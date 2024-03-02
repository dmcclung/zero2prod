//! tests/api/utils.rs

use anyhow::Result;

use sqlx::postgres::PgPoolOptions;
use zero2prod::app::Application;
use zero2prod::config::Config;

pub async fn spawn_app() -> Result<String> {
    let config = Config::new();

    reset_subscriptions(&config).await?;

    let app = Application::build(&config, "127.0.0.1:0".into()).await?;
    let address = format!("http://127.0.0.1:{}", app.port());
    let _ = tokio::spawn(app.run_until_stopped());

    Ok(address)
}

async fn reset_subscriptions(config: &Config) -> Result<()> {
    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;
    sqlx::query!("DELETE FROM subscriptions")
        .execute(&pool)
        .await?;

    Ok(())
}
