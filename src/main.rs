use std::net::TcpListener;

use anyhow::Result;
use sqlx::PgPool;
use zero2prod::{configuration::Settings, startup::run};

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let config = Settings::from_env().expect("failed to load .env file");
    let db_url = config.database.connection_string();
    let db_pool =
        PgPool::connect(&db_url).await.expect("failed to connect to database");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    println!("Server is running at port: {}", config.application_port);
    Ok(run(listener, db_pool)?.await?)
}
