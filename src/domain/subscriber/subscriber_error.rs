//! src/domain/subscriber/subscriber_error.rs

use actix_web::{error::ResponseError, HttpResponse};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum SubscriberError {
    ParseError(String),
    DatabaseError(sqlx::Error),
    EmailError(String),
}

impl Display for SubscriberError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            SubscriberError::ParseError(e) => write!(f, "Parse Error: {}", e),
            SubscriberError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            SubscriberError::EmailError(e) => write!(f, "Error sending email: {}", e),
        }
    }
}

impl ResponseError for SubscriberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubscriberError::ParseError(ref message) => HttpResponse::BadRequest().json(message),
            SubscriberError::DatabaseError(ref _error) => {
                HttpResponse::InternalServerError().finish()
            }
            SubscriberError::EmailError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}
