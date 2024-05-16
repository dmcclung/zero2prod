//! src/routes/newsletter.rs

use crate::{
    auth::validate_request,
    domain::{
        newsletter::{Newsletter, NewsletterError},
        subscriber::{Subscriber, SubscriberError},
    },
    email::{Email, EmailService},
};
use actix_web::{web, HttpResponse};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tracing::{info, instrument, Instrument};
use uuid::Uuid;

#[instrument(
    name = "Publish a newsletter issue",
    skip(json, pool, email_service, request),
    fields(
        request_id = %Uuid::new_v4(),
        username=tracing::field::Empty,
        user_id=tracing::field::Empty
    )
)]
pub async fn publish_newsletter(
    json: web::Json<Newsletter>,
    pool: web::Data<Pool<Postgres>>,
    email_service: web::Data<Arc<dyn EmailService + Send + Sync>>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = validate_request(request, pool.get_ref()).await?;

    tracing::Span::current().record("user_id", &tracing::field::display(user_id));

    let confirmed_emails: Vec<Subscriber> = sqlx::query_as!(
        Subscriber,
        r#"
        SELECT email, name, status
        FROM subscriptions
        WHERE status = 'confirmed'
        "#
    )
    .fetch_all(pool.get_ref())
    .instrument(tracing::info_span!("get confirmed emails query"))
    .await
    .map_err(NewsletterError::DatabaseError)?;

    info!("Confirmed email addresses: {}", confirmed_emails.len());

    for confirmed_email in confirmed_emails {
        let email = Email {
            to: &confirmed_email.email.to_string(),
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
