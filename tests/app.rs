//! tests/app.rs

use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

pub async fn spawn() -> Result<String, sqlx::Error> {
    let config = zero2prod::config::Config::new();
    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::migrate!().run(&pool).await?;
    sqlx::query!("DELETE FROM subscriptions")
        .execute(&pool)
        .await?;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener, pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    Ok(format!("http://127.0.0.1:{}", port))
}
