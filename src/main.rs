use std::net::TcpListener;
use sqlx::Pool;
use sqlx::postgres::Postgres;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
    let _pool = Pool::<Postgres>::connect("postgres://admin:admin@localhost:5432/newsletter").await?;

    println!("Created postgres pool");

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    Ok(run(listener)?.await?)
}
