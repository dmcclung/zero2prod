use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use sqlx::postgres::PgPoolOptions;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {  
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        std::io::stdout
    );

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    let config = zero2prod::config::Config::new();

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::migrate!().run(&pool).await?;
    
    let addr = format!("[::]:{}", config.port);

    let listener = TcpListener::bind(addr)?;
    Ok(zero2prod::run(listener, pool)?.await?)
}
