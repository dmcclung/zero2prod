//! src/subscriber_email.rs

#[derive(serde::Deserialize, Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> SubscriberEmail {
        // TODO: validation goes here
        SubscriberEmail(s)
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