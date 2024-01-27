use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let settings = zero2prod::config::get_settings();

    let pool = PgPoolOptions::new().connect("postgres://admin:admin@localhost:5432/newsletter").await?;
    sqlx::migrate!().run(&pool).await?;
    
    let addr = format!("[::]:{}", settings.port);

    let listener = TcpListener::bind(addr)?;
    Ok(zero2prod::run(listener)?.await?)
}
