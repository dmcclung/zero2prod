//! tests/api/utils.rs

use anyhow::Result;

use lettre::Message;
use reqwest::Response;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use zero2prod::app::Application;
use zero2prod::config::Config;
use zero2prod::email::{EmailSender, EmailService};

pub struct TestApp {
    address: String,
    pool: Pool<Postgres>,
}

struct MockEmailSender {
    sent_messages: Vec<Message>,
}

impl MockEmailSender {
    fn new() -> Self {
        Self {
            sent_messages: Vec::new(),
        }
    }
}

impl EmailSender for MockEmailSender {
    fn send(
        &mut self,
        _port: u16,
        _host: &str,
        _creds: lettre::transport::smtp::authentication::Credentials,
        message: lettre::Message,
    ) -> Result<()> {
        self.sent_messages.push(message);
        Ok(())
    }
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

    pub async fn post_subscriptions(
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
}

pub async fn spawn_app() -> Result<TestApp> {
    let config = Config::new();

    let email_sender = &mut MockEmailSender::new();
    let email_service = EmailService::new(config.smtp_config.clone(), email_sender);

    let app = Application::build(&config, "127.0.0.1:0".into(), email_service).await?;
    let address = format!("http://127.0.0.1:{}", app.port());
    let _ = tokio::spawn(app.run_until_stopped());

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;

    Ok(TestApp { address, pool })
}
