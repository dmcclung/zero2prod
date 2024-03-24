use std::sync::Mutex;
use zero2prod::email::{Email, EmailService};

#[derive(Debug)]
pub struct MockEmailService {
    pub sent_messages: Mutex<Vec<(String, String, String)>>,
}

impl Default for MockEmailService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockEmailService {
    pub fn new() -> Self {
        Self {
            sent_messages: Mutex::new(Vec::new()),
        }
    }
}

impl EmailService for MockEmailService {
    fn send(&self, message: Email) -> Result<(), String> {
        self.sent_messages.lock().unwrap().push((
            message.to.to_owned(),
            message.html.to_owned(),
            message.plaintext.to_owned(),
        ));
        Ok(())
    }
}
