//! src/routes/newsletter.rs

use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::{info, instrument, Instrument};
use uuid::Uuid;

use crate::{domain::subscriber::SubscriberError, email::{EmailService, Email}};

#[derive(Deserialize)]
pub struct NewsletterJson {
    pub newsletter: String,
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

    let email = Email {
        to: "",
        html: "", 
        from: "", 
        subject: "", 
        reply_to: "", 
        plaintext: &json.newsletter 
    };

    email_service.send(email)?;
    Ok(HttpResponse::Ok().finish())
}
