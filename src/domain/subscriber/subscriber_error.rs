//! src/subscriber_error.rs

#[derive(Debug)]
pub enum SubscriberError {
    ParseError(String),
}

impl std::fmt::Display for SubscriberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SubscriberError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl std::error::Error for SubscriberError {}