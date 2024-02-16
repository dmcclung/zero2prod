//! src/subscriber_email.rs

use regex::Regex;

#[derive(serde::Deserialize, Debug)]
pub struct SubscriberEmail(String);

#[derive(Debug)]
pub enum SubscriberError {
    ParseError(String)
}

impl std::fmt::Display for SubscriberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SubscriberError::ParseError(e) => write!(f, "Parse Error: {}", e),            
        }
    }
}

impl std::error::Error for SubscriberError{}

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, SubscriberError> {
        let email_regex = Regex::new(
            r"^(?i)[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$"
        ).unwrap();

        if !email_regex.is_match(&s) {
            return Err(SubscriberError::ParseError("Invalid email".into()));
        }

        Ok(SubscriberEmail(s))
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<String> for SubscriberEmail {
    fn as_ref(&self) -> &String {
        let SubscriberEmail(inner) = self;
        inner
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::subscriber::SubscriberEmail;
    use claims::{ assert_ok, assert_err };

    #[test]
    fn test_whitespace_email () {
        assert_err!(SubscriberEmail::parse(" ".into()));
    }

    #[test]
    fn test_missing_local_part () {
        assert_err!(SubscriberEmail::parse("@missinglocalpart.com".into()));
    }

    #[test]
    fn test_good_email () {
        assert_ok!(SubscriberEmail::parse("dev@zero2prod.xyz".into()));
    }
}