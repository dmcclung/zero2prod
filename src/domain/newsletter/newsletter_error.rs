//! src/domain/subscriber/subscriber_error.rs

use actix_web::{error::ResponseError, HttpResponse};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum NewsletterError {
    PublishError(String),
    DatabaseError(sqlx::Error),
    EmailError(String),
}

impl Display for NewsletterError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            NewsletterError::PublishError(e) => write!(f, "Publish Error: {}", e),
            NewsletterError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            NewsletterError::EmailError(e) => write!(f, "Error sending email: {}", e),
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
        }
    }
}
