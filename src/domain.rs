//! src/domain.rs

#[derive(serde::Deserialize, Debug)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {

        return SubscriberName(s);
    }
}

impl std::fmt::Display for SubscriberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<String> for SubscriberName {
    fn as_ref(&self) -> &String {
        let SubscriberName(inner) = self;
        return inner;
    }
}