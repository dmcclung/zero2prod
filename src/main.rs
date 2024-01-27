use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new().connect("postgres://admin:admin@localhost:5432/newsletter").await?;
    sqlx::migrate!().run(&pool).await?;

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    Ok(run(listener)?.await?)
}
