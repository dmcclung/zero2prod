use std::net::TcpListener;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let config = zero2prod::config::Config::new();

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::migrate!().run(&pool).await?;
    
    let addr = format!("[::]:{}", config.port);

    let listener = TcpListener::bind(addr)?;
    Ok(zero2prod::run(listener, pool)?.await?)
}
