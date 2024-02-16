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
        if s.trim().is_empty() {
            return Err(SubscriberError::ParseError("Empty name".into()))
        }

        if s.len() > 50 {
            return Err(SubscriberError::ParseError("Length greater than 50".into()))
        }

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

#[cfg(test)]
mod tests {
    use crate::domain::subscriber::SubscriberName;
    use claims::{ assert_ok, assert_err };

    #[test]
    fn test_empty_name () {
        assert_err!(SubscriberName::parse("".into()));        
    }

    #[test]
    fn test_long_name () {
        assert_err!(SubscriberName::parse("a".repeat(51)));        
    }

    #[test]
    fn test_good_name () {
        assert_ok!(SubscriberName::parse("zero2prod".into()));
    }

}