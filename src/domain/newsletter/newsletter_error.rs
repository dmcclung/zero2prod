//! src/domain/subscriber/error.rs

use actix_web::{error::ResponseError, HttpResponse};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum NewsletterError {
    PublishError(String),
    DatabaseError(sqlx::Error),
    EmailError(String),
    AuthError(),
    HasherError(argon2::Error),
}

impl Display for NewsletterError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            NewsletterError::PublishError(e) => write!(f, "Publish Error: {}", e),
            NewsletterError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            NewsletterError::EmailError(e) => write!(f, "Error sending email: {}", e),
            NewsletterError::AuthError() => write!(f, "Unauthorized"),
            NewsletterError::HasherError(e) => write!(f, "Error hashing password: {}", e),
        }
    }
}

impl ResponseError for NewsletterError {
    fn error_response(&self) -> HttpResponse {
        match self {
            NewsletterError::PublishError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            NewsletterError::DatabaseError(ref error) => {
                HttpResponse::InternalServerError().json(error.to_string())
            }
            NewsletterError::EmailError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            NewsletterError::AuthError() => HttpResponse::Unauthorized().finish(),
            NewsletterError::HasherError(ref _error) => HttpResponse::Unauthorized().finish(),
        }
    }
}
