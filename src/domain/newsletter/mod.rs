mod newsletter_error;

pub use newsletter_error::*;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Newsletter {
    pub html: String,
    pub text: String,
    pub subject: String,
}

pub struct ConfirmedSubscriber {
    pub email: String,
}
