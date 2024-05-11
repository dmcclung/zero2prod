//! src/auth.rs

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials(String),
    UnexpectedError(String),
}
