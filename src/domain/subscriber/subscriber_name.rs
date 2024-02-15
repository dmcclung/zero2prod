#[derive(serde::Deserialize, Debug)]
pub struct SubscriberName(String);

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

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, SubscriberError> {
        // TODO: validation goes here
        Ok(SubscriberName(s))
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
        inner
    }
}