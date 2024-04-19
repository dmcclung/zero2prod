mod error;

pub use error::*;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Newsletter {
    pub html: String,
    pub text: String,
    pub subject: String,
}
