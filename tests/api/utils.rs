//! tests/api/utils.rs

use anyhow::Result;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use zero2prod::app::Application;
use zero2prod::config::Config;

pub struct TestApp {
    address: String,
    pool: Pool<Postgres>,
}

impl TestApp {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub async fn reset_subscriptions(&self) -> Result<()> {
        sqlx::query!("DELETE FROM subscriptions")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_subscription(&self) -> (String, String) {
        let subscription = sqlx::query!("SELECT email, name FROM subscriptions")
            .fetch_one(&self.pool)
            .await
            .expect("Failed to fetch saved subscription");

        (subscription.email, subscription.name)
    }
}

pub async fn spawn_app() -> Result<TestApp> {
    let config = Config::new();

    let app = Application::build(&config, "127.0.0.1:0".into()).await?;
    let address = format!("http://127.0.0.1:{}", app.port());
    let _ = tokio::spawn(app.run_until_stopped());

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;

    Ok(TestApp { address, pool })
}
