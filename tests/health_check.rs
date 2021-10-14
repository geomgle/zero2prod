use std::net::TcpListener;

use pretty_assertions::assert_eq;
use reqwest;
use sqlx::PgPool;
use zero2prod::startup::run;

lazy_static::lazy_static! {
    static ref APPLICATION_PORT: String =  dotenv::var("APPLICATION_PORT").unwrap();
    static ref DATABASE_URL: String =  dotenv::var("DATABASE_URL").unwrap();
}

struct TestApp {
    address: String,
    db_pool: PgPool,
}

#[actix_rt::test]
pub(crate) async fn check_health_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(25), response.content_length());
}

async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("0.0.0.0:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://0.0.0.0:{}", port);
    let db_pool = PgPool::connect(&DATABASE_URL)
        .await
        .expect("failed to connect to postgres");

    let server =
        run(listener, db_pool.clone()).expect("failed to spawn our app");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(400, response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.", error_message);
    }
}
