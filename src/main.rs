use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: use config.rs to pull in the settings for all this
    let settings = zero2prod::config::Settings{
        port: 3000,
    };

    let pool = PgPoolOptions::new().connect("postgres://admin:admin@localhost:5432/newsletter").await?;
    sqlx::migrate!().run(&pool).await?;
    
    let addr = format!("[::]:{}", settings.port);

    let listener = TcpListener::bind(addr)?;
    Ok(zero2prod::run(listener)?.await?)
}
