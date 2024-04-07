//! tests/api/test_app.rs

use std::sync::Arc;

use reqwest::Response;
use serde_json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use zero2prod::app::Application;
use zero2prod::config::Config;

use crate::mocks::MockEmailService;

pub struct TestApp {
    address: String,
    pool: Pool<Postgres>,
    email_service: Arc<MockEmailService>,
}

impl TestApp {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub async fn get_subscription(
        &self,
        subscriber_name: &String,
        subscriber_email: &String,
    ) -> Uuid {
        let subscriber = sqlx::query!(
            "SELECT id FROM subscriptions WHERE name = $1 AND email = $2",
            subscriber_name,
            subscriber_email
        )
        .fetch_one(&self.pool)
        .await
        .expect("Failed to fetch saved subscription");

        subscriber.id
    }

    pub async fn get_subscription_token(&self, subscriber_id: Uuid) -> String {
        let subscription_token = sqlx::query!(
            "SELECT subscription_token FROM subscription_tokens WHERE subscriber_id = $1",
            subscriber_id
        )
        .fetch_one(&self.pool)
        .await
        .expect("Failed to fetch subscription token");

        subscription_token.subscription_token
    }

    pub async fn get_confirmed_subscriptions(&self) -> usize {
        let confirmed_count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM subscriptions
            WHERE status = 'confirmed'
            "#
        )
        .fetch_one(&self.pool)
        .await
        .expect("Failed to fetch confirmed subscription count");

        confirmed_count.count.expect("Error getting count") as usize
    }

    pub async fn create_subscription(
        &self,
        name: String,
        email: String,
    ) -> Result<Response, reqwest::Error> {
        let body = format!("name={}&email={}", name, email);

        let client = reqwest::Client::new();
        client
            .post(&format!("{}/subscriptions", self.address()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
    }

    pub async fn confirm_subscription(&self, token: &str) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        client
            .get(&format!("{}/confirm?token={}", self.address(), token))
            .send()
            .await
    }

    pub async fn publish_newsletter(
        &self,
        html: String,
        text: String,
        subject: String,
    ) -> Result<Response, reqwest::Error> {
        let newsletter = serde_json::json!({
            "html": html,
            "text": text,
            "subject": subject
        });
        let client = reqwest::Client::new();
        client
            .post(&format!("{}/newsletter", self.address()))
            .header("Content-Type", "application/json")
            .body(newsletter.to_string())
            .send()
            .await
    }

    pub async fn confirm_subscription_no_token(&self) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        client
            .get(&format!("{}/confirm", self.address()))
            .send()
            .await
    }

    pub fn get_sent_emails(&self) -> Vec<(String, String, String)> {
        self.email_service.sent_messages.lock().unwrap().to_vec()
    }
}

pub async fn spawn() -> Result<TestApp, String> {
    let config = Config::new();

    let email_service = Arc::new(MockEmailService::new());

    let app = Application::build(&config, "127.0.0.1:0".into(), email_service.clone()).await?;
    let address = format!("http://127.0.0.1:{}", app.port());
    let _ = tokio::spawn(app.run_until_stopped());

    let pool = PgPoolOptions::new()
        .connect(&config.db_config.url)
        .await
        .map_err(|e| format!("Error connecting to db: {}", e))?;

    Ok(TestApp {
        address,
        pool,
        email_service,
    })
}
