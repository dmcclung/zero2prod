use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let config = zero2prod::config::Config::new();

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::migrate!().run(&pool).await?;
    
    let addr = format!("[::]:{}", config.port);

    let listener = TcpListener::bind(addr)?;
    Ok(zero2prod::run(listener)?.await?)
}
