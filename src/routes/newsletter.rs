//! src/routes/newsletter.rs

use std::{
    fmt::{Display, Error, Formatter},
    sync::Arc,
};

use actix_web::{http::header::HeaderMap, web, HttpResponse, ResponseError};
use base64::Engine;
use secrecy::{ExposeSecret, Secret};
use sqlx::{Pool, Postgres};
use tracing::{info, instrument, Instrument};
use uuid::Uuid;

use crate::{
    domain::{
        newsletter::{ConfirmedSubscriber, Newsletter, NewsletterError},
        subscriber::SubscriberError,
    },
    email::{Email, EmailService},
};

struct Credentials {
    username: String,
    password: Secret<String>,
}

fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, String> {
    let header_value = headers
        .get("Authorization")
        .ok_or("Authorization header not found")?
        .to_str()
        .map_err(|e| format!("Authorization header to string error: {}", e))?;

    let base64encoded = header_value
        .strip_prefix("Basic ")
        .ok_or("Authorization scheme not Basic")?;

    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64encoded)
        .map_err(|e| format!("Decoding authorization header: {}", e))?;

    let decoded_creds = String::from_utf8(decoded_bytes)
        .map_err(|e| format!("Stringifying decoded authorization header: {}", e))?;

    let mut credentials = decoded_creds.splitn(2, ':');

    let username = credentials.next().ok_or("Username missing")?.to_string();
    let password = credentials.next().ok_or("Password missing")?.to_string();

    Ok(Credentials {
        username,
        password: Secret::new(password),
    })
}

#[instrument(
    skip(json, pool, email_service),
    fields(
        request_id = %Uuid::new_v4()
    )
)]
pub async fn publish_newsletter(
    json: web::Json<Newsletter>,
    pool: web::Data<Pool<Postgres>>,
    email_service: web::Data<Arc<dyn EmailService + Send + Sync>>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let credentials =
        basic_authentication(request.headers()).map_err(NewsletterError::PublishError)?;

    if credentials.username != "admin" || credentials.password.expose_secret() != "password" {
        return Ok(HttpResponse::Unauthorized().finish());
    }

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
