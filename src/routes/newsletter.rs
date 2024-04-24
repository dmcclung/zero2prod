//! src/routes/newsletter.rs

use std::{
    fmt::{Display, Error, Formatter},
    sync::Arc,
};

use actix_web::{http::header::HeaderMap, web, HttpResponse, ResponseError};
use base64::Engine;
use sqlx::{Pool, Postgres};
use tracing::{error, info, instrument, Instrument};
use uuid::Uuid;

use sha3::Digest;

use crate::{
    domain::{
        newsletter::{Newsletter, NewsletterError},
        subscriber::{Subscriber, SubscriberError},
    },
    email::{Email, EmailService},
};

struct Credentials {
    username: String,
    password_hash: String,
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

    let password_hash = sha3::Sha3_256::digest(
        password.as_bytes());

    let password_hash = format!("{:x}", password_hash);

    Ok(Credentials {
        username,
        password_hash,
    })
}

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
    let credentials = basic_authentication(request.headers()).map_err(|e| {
        error!(e);
        NewsletterError::AuthError()
    })?;

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    let user_row = sqlx::query!(
        r#"
        SELECT id FROM users WHERE username = $1 AND password_hash = $2
        "#,
        credentials.username,
        credentials.password_hash
    )
    .fetch_optional(pool.get_ref())
    .instrument(tracing::info_span!("lookup user"))
    .await
    .map_err(|e| {
        error!("{}", e);
        NewsletterError::AuthError()
    })?;

    let user_id = user_row
        .map(|r| r.id)
        .ok_or_else(NewsletterError::AuthError)?;

    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

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
