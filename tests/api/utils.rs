//! tests/api/utils.rs

use anyhow::Result;

use sqlx::postgres::PgPoolOptions;
use zero2prod::app::Application;

pub async fn spawn_app() -> Result<String> {
    let config = zero2prod::config::Config::new();

    let app = Application::build(&config, "127.0.0.1:0".into()).await?;

    let _ = tokio::spawn(app.server);

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::query!("DELETE FROM subscriptions")
        .execute(&pool)
        .await?;

    Ok(format!("http://127.0.0.1:{}", app.port))
}
