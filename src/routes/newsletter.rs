//! src/routes/newsletter.rs

use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::{info, instrument, Instrument};
use uuid::Uuid;

use crate::{
    domain::subscriber::SubscriberError,
    email::{Email, EmailService},
};

#[derive(Deserialize, Clone)]
pub struct NewsletterJson {
    pub html: String,
    pub text: String,
    pub subject: String,
}

#[instrument(
    skip(json, pool, email_service),
    fields(
        request_id = %Uuid::new_v4()
    )
)]
pub async fn publish_newsletter(
    json: web::Json<NewsletterJson>,
    pool: web::Data<Pool<Postgres>>,
    email_service: web::Data<Arc<dyn EmailService + Send + Sync>>,
) -> Result<HttpResponse, actix_web::Error> {
    let confirmed_emails = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#
    )
    .fetch_all(pool.get_ref())
    .instrument(tracing::info_span!("get confirmed emails query"))
    .await
    .map_err(SubscriberError::DatabaseError)?;

    info!("Confirmed email addresses: {}", confirmed_emails.len());

    for confirmed_email in confirmed_emails {
        let email = Email {
            to: &confirmed_email.email,
            html: &json.html,
            from: "",
            subject: &json.subject,
            reply_to: "",
            plaintext: &json.text,
        };

        email_service
            .send(email)
            .map_err(SubscriberError::EmailError)?
    }

    Ok(HttpResponse::Ok().finish())
}
