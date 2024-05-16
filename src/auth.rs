//! src/auth.rs

use std::fmt::{Display, Error, Formatter};

use actix_web::{http::header::HeaderMap, HttpResponse, ResponseError};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::Engine;
use secrecy::{ExposeSecret, Secret};
use sqlx::{Pool, Postgres};
use tokio::task;
use tracing::{error, Instrument};
use uuid::Uuid;

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UnexpectedError(String),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidCredentials => HttpResponse::Unauthorized().finish(),
            AuthError::UnexpectedError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::UnexpectedError(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
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
    let password = Secret::from(credentials.next().ok_or("Password missing")?.to_string());

    Ok(Credentials { username, password })
}

pub async fn validate_request(
    request: actix_web::HttpRequest,
    pool: &Pool<Postgres>,
) -> Result<Uuid, AuthError> {
    let credentials = basic_authentication(request.headers()).map_err(|e| {
        error!(e);
        AuthError::InvalidCredentials
    })?;

    validate_credentials(credentials, pool).await
}

pub async fn validate_credentials(
    credentials: Credentials,
    pool: &Pool<Postgres>,
) -> Result<Uuid, AuthError> {
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    let user = sqlx::query!(
        r#"
        SELECT id, password_hash FROM users WHERE username = $1
        "#,
        credentials.username,
    )
    .fetch_optional(pool)
    .instrument(tracing::info_span!("lookup user"))
    .await
    .map_err(|e| {
        error!("{}", e);
        AuthError::InvalidCredentials
    })?
    .ok_or(AuthError::InvalidCredentials)?;

    let handle = task::spawn_blocking(move || {
        let argon2 = Argon2::default();

        let parsed_hash =
            PasswordHash::new(&user.password_hash).map_err(|_| AuthError::InvalidCredentials)?;

        argon2
            .verify_password(
                credentials.password.expose_secret().as_bytes(),
                &parsed_hash,
            )
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok::<(), AuthError>(())
    });

    handle
        .await
        .map_err(|e| AuthError::UnexpectedError(e.to_string()))??;

    Ok(user.id)
}
