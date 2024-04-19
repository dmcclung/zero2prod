//! src/domain/subscriber/subscriber_error.rs

use actix_web::{error::ResponseError, HttpResponse};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum SubscriberError {
    ParseError(String),
    DatabaseError(sqlx::Error),
    EmailError(String),
    InvalidToken(String),
}

impl Display for SubscriberError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            SubscriberError::ParseError(e) => write!(f, "Parse Error: {}", e),
            SubscriberError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            SubscriberError::EmailError(e) => write!(f, "Error sending email: {}", e),
            SubscriberError::InvalidToken(e) => write!(f, "Invalid token: {}", e),
        }
    }
}

impl ResponseError for SubscriberError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubscriberError::ParseError(ref message) => HttpResponse::BadRequest().json(message),
            SubscriberError::DatabaseError(ref error) => {
                HttpResponse::InternalServerError().json(error.to_string())
            }
            SubscriberError::EmailError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            SubscriberError::InvalidToken(ref token) => {
                HttpResponse::BadRequest().json(format!("Invalid token: {}", token))
            }
        }
    }
}
