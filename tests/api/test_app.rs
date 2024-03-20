//! tests/api/utils.rs

use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::Response;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use zero2prod::app::Application;
use zero2prod::config::Config;
use zero2prod::email::mocks::MockEmailSender;
use zero2prod::email::EmailService;

pub struct TestApp<'a> {
    address: String,
    pool: Pool<Postgres>,
    mock_email_sender: &'a MockEmailSender,
}

impl<'a> TestApp<'a> {
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

    pub async fn confirm_subscription(&self) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();
        client
            .get(&format!("{}/confirm?token={}", self.address(), "12345efg"))
            .send()
            .await
    }

    pub fn get_emails_sent(&self) -> usize {
        self.mock_email_sender.sent_messages.lock().unwrap().len()
    }

    pub fn email_body_contains(&self, substr: &str) -> bool {
        let sent_messages = self.mock_email_sender.sent_messages.lock().unwrap();

        if let Some(message) = sent_messages.get(0) {
            let message_formatted = message.formatted();
            let message_body = std::str::from_utf8(&message_formatted).unwrap();
            message_body.contains(substr)
        } else {
            panic!("Message not sent");
        }
    }
}

pub async fn spawn() -> Result<TestApp<'static>> {
    let config = Config::new();

    static EMAIL_SENDER: Lazy<MockEmailSender> = Lazy::new(|| MockEmailSender::new());
    let email_service = EmailService::new(config.smtp_config.clone(), &*EMAIL_SENDER);

    let app = Application::build(&config, "127.0.0.1:0".into(), email_service).await?;
    let address = format!("http://127.0.0.1:{}", app.port());
    let _ = tokio::spawn(app.run_until_stopped());

    let pool = PgPoolOptions::new().connect(&config.db_config.url).await?;

    Ok(TestApp {
        address,
        pool,
        mock_email_sender: &*EMAIL_SENDER,
    })
}
