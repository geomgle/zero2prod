use std::net::TcpListener;

use anyhow::Result;
use once_cell::sync::Lazy;
use pretty_assertions::assert_eq;
use reqwest;
use sqlx::PgPool;
use zero2prod::{
    configuration::Settings,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        init_subscriber(get_subscriber("test", "info", std::io::stdout));
    } else {
        init_subscriber(get_subscriber("test", "info", std::io::sink));
    }
});

struct TestApp {
    address: String,
    db_pool: PgPool,
}

#[actix_rt::test]
async fn check_health_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(20), response.content_length());
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener =
        TcpListener::bind("0.0.0.0:0").expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://0.0.0.0:{}", port);

    let config = Settings::from_env().expect("failed to load .env file");
    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect pgpool");
    sqlx::query!(
        r#"
        CREATE TEMP TABLE subscriptions
        AS SELECT * FROM subscriptions LIMIT 0
    "#
    )
    .execute(&db_pool)
    .await
    .expect("failed to create temp table");

    let server =
        run(listener, db_pool.clone()).expect("failed to spawn our app");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool,
    }
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

    let saved = sqlx::query!(
        r#"
        SELECT email, name 
            FROM subscriptions 
            WHERE email = 'ursula_le_guin@gmail.com'
        "#
    )
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
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message,
        );
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty(
) -> Result<()> {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=la_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .map_err(|e| {
                println!(
                    "\x1b[1m\x1b[38;2;235;125;125m failed to execute request: \
                    \x1b[0m\x1b[38;2;235;125;125m{:#?}\x1b[0m",
                    e
                );
                e
            })?;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            error_message,
        );
    }
    Ok(())
}
