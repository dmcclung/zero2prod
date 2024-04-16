//! src/routes/newsletter.rs

use std::{
    fmt::{Display, Error, Formatter},
    sync::Arc,
};

use actix_web::{http::header::HeaderMap, web, HttpResponse, ResponseError};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::{info, instrument, Instrument};
use uuid::Uuid;
use secrecy::Secret;

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

struct ConfirmedSubscriber {
    email: String,
}

struct Credentials {
    _username: String,
    _password: Secret<String>
}

fn basic_authentication(_headers: &HeaderMap) -> Result<Credentials, String> {
    todo!("Not implemented");
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
    request: actix_web::HttpRequest
) -> Result<HttpResponse, actix_web::Error> {
    let _credentials = basic_authentication(request.headers());
    let confirmed_emails: Vec<ConfirmedSubscriber> = sqlx::query_as!(
        ConfirmedSubscriber,
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

#[derive(Debug)]
pub enum PublishError {
    DatabaseError(sqlx::Error),
    EmailError(String),
}

impl Display for PublishError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            PublishError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            PublishError::EmailError(e) => write!(f, "Error sending email: {}", e),
        }
    }
}

impl ResponseError for PublishError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PublishError::DatabaseError(ref error) => {
                HttpResponse::InternalServerError().json(error.to_string())
            }
            PublishError::EmailError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}
