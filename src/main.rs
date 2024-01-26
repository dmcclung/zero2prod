use std::net::TcpListener;
use sqlx::Pool;
use sqlx::postgres::Postgres;

use zero2prod::run;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the connection setup by building the connection string from a function
    // sqlx::migrate!().run(<&your_pool OR &mut your_connection>).await?;
    let _pool = Pool::<Postgres>::connect("postgres://").await?;

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    Ok(run(listener)?.await?)
}
