//! src/domain/subscriber/subscriber_email.rs

use crate::domain::subscriber::SubscriberError;
use regex::Regex;

#[derive(serde::Deserialize, Debug)]
pub struct SubscriberEmail(String);

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

impl From<String> for SubscriberEmail {
    fn from(value: String) -> Self {
        SubscriberEmail::parse(value).expect("Invalid subscriber email")
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
    use claims::assert_err;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn test_whitespace_email() {
        assert_err!(SubscriberEmail::parse(" ".into()));
    }

    #[test]
    fn test_missing_local_part() {
        assert_err!(SubscriberEmail::parse("@missinglocalpart.com".into()));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn test_valid_email(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
