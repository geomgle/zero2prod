use std::net::TcpListener;

use anyhow::Result;
use sqlx::PgPool;
use zero2prod::startup::run;

lazy_static::lazy_static! {
    static ref APPLICATION_PORT: String =  dotenv::var("APPLICATION_PORT").unwrap();
    static ref DATABASE_URL: String =  dotenv::var("DATABASE_URL").unwrap();
}

#[actix_web::main]
async fn main() -> Result<()> {
    let db_pool = PgPool::connect(&DATABASE_URL)
        .await
        .expect("failed to connect to database");
    let address = format!("0.0.0.0:{}", *APPLICATION_PORT);
    let listener = TcpListener::bind(address)?;

    println!("Server is running at port: {}", *APPLICATION_PORT);
    Ok(run(listener, db_pool)?.await?)
}
