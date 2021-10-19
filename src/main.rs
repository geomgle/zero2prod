use std::net::TcpListener;

use sqlx::PgPool;
use tracing_subscriber::layer::SubscriberExt;
use zero2prod::{configuration::Settings, startup::run};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_subscriber<'a>(
    name: &'a str,
    env_filter: &'a str,
) -> impl tracing::Subscriber + Send + Sync {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(env_filter));
    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(
        name.into(),
        std::io::stdout,
    );
    tracing_subscriber::Registry::default()
        .with(env_filter)
        .with(tracing_bunyan_formatter::JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing_log::LogTracer::init().expect("failed to set logger");
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set subscriber");
}

#[actix_web::main]
async fn main() -> Result<()> {
    init_subscriber(get_subscriber("zero2prod", "info"));
    let config = Settings::from_env().expect("failed to load .env file");

    let db_url = config.database.connection_string();
    let db_pool =
        PgPool::connect(&db_url).await.expect("failed to connect to database");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    println!("Server is running at port: {}", config.application_port);
    Ok(run(listener, db_pool)?.await?)
}
