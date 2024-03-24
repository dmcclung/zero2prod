//! src/email.rs

use lettre::message::{Mailbox, MultiPart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};

use crate::config::SmtpConfig;


/// Represents an email message.
///
/// This struct includes all the common fields required to construct an email,
/// including the recipient, sender, subject, and the body of the message, which
/// can be provided in both HTML and plaintext formats for compatibility with
/// different email clients.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use zero2prod::email::Email;
///
/// let email = Email {
///     to: "recipient@example.com",
///     from: "sender@example.com",
///     subject: "Greetings!",
///     reply_to: "no-reply@example.com",
///     html: "<h1>Hello</h1><p>How are you?</p>",
///     plaintext: "Hello\nHow are you?",
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Email<'a> {
    /// The recipient's email address.
    pub to: &'a str,
    /// The HTML content of the email message. This field allows the inclusion of
    /// HTML tags for formatting purposes.
    pub html: &'a str,
    /// The sender's email address.
    pub from: &'a str,
    /// The subject of the email message.
    pub subject: &'a str,
    /// The email address for reply-to field, which indicates where replies to the
    /// email should be sent.
    pub reply_to: &'a str,
    /// The plaintext content of the email message. This field is used for email
    /// clients that do not support HTML content or as a fallback.
    pub plaintext: &'a str,
}

pub trait EmailService {
    fn send(&self, email: Email) -> Result<(), String>;
}

#[derive(Debug)]
pub struct EmailServiceImpl {
    config: SmtpConfig,
    smtp_transport: SmtpTransport,
}

impl EmailServiceImpl {
    pub fn new(config: SmtpConfig) -> Self {
        let tls_parameters = TlsParameters::builder(config.host.clone()).build().unwrap();

        let creds = Credentials::new(config.user.clone(), config.password.clone());

        let smtp_transport = SmtpTransport::relay(&config.host).unwrap()
            .tls(Tls::Required(tls_parameters))
            .port(config.port)
            .credentials(creds)
            .build();

        Self {
            config,
            smtp_transport,
        }
    }
}

impl EmailService for EmailServiceImpl {
    fn send(&self, email: Email) -> Result<(), String> {
        let to: Mailbox = email.to.parse().map_err(|e| format!("Error: {}", e)).unwrap();
        let from: Mailbox = if email.from.is_empty() {
            self.config.default_sender.parse().map_err(|e| format!("Error: {}", e)).unwrap()
        } else {
            email.from.parse().map_err(|e| format!("Error: {}", e)).unwrap()
        };

        let mut message_builder = Message::builder().from(from).to(to).subject(email.subject);

        if !email.reply_to.is_empty() {
            let reply_to: Mailbox = email.reply_to.parse().map_err(|e| format!("Error: {}", e)).unwrap();

            message_builder = message_builder.reply_to(reply_to);
        }

        let message = message_builder.multipart(MultiPart::alternative_plain_html(
            email.plaintext.to_string(),
            email.html.to_string(),
        )).map_err(|e| format!("Error: {}", e)).unwrap();

        match self.smtp_transport.send(&message) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error sending email {}", e))
        }
    }
}

pub mod mocks {
    use std::sync::Mutex;

    use super::{Email, EmailService};

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
        fn send(
            &self,
            message: Email,
        ) -> Result<(), String> {
            
            self.sent_messages.lock().unwrap().push((message.to.to_owned(), message.html.to_owned(), message.plaintext.to_owned()));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;
    use fake::faker::internet::en::{DomainSuffix, Password, SafeEmail, Username};
    use fake::faker::lorem::en::Sentence;
    use fake::faker::number::en::NumberWithFormat;
    use fake::Fake;

    use std::env::{remove_var, set_var};

    use crate::email::mocks::MockEmailService;

    use super::{Email, EmailService, SmtpConfig};

    #[test]
    fn smtp_config_from_env() {
        let hostname = generate_hostname();
        let username: String = Username().fake();
        let port: String = NumberWithFormat("###").fake();
        let password: String = Password(8..16).fake();
        let default_sender: String = SafeEmail().fake();

        set_var("EMAIL_HOST", hostname.clone());
        set_var("EMAIL_USER", username.clone());
        set_var("EMAIL_PORT", port.clone());
        set_var("EMAIL_PASSWORD", password.clone());
        set_var("EMAIL_DEFAULT_SENDER", default_sender.clone());

        let smtp_config = SmtpConfig::parse_from_env();

        remove_var("EMAIL_HOST");
        remove_var("EMAIL_USER");
        remove_var("EMAIL_PORT");
        remove_var("EMAIL_PASSWORD");
        remove_var("EMAIL_DEFAULT_SENDER");

        assert_eq!(hostname, smtp_config.host);
        assert_eq!(username, smtp_config.user);
        assert_eq!(port.parse::<u16>().unwrap(), smtp_config.port);
        assert_eq!(password, smtp_config.password);
        assert_eq!(default_sender, smtp_config.default_sender);
    }

    fn generate_hostname() -> String {
        let domain: String = Username().fake();
        let domain_suffix: String = DomainSuffix().fake();
        format!("smtp.{}.{}", domain, domain_suffix)
    }

    #[test]
    fn send_valid_email() {
        let email_service = &MockEmailService::new();

        let to: String = SafeEmail().fake();
        let from: String = SafeEmail().fake();
        let subject: String = Sentence(1..5).fake();
        let plaintext: String = Sentence(1..10).fake();

        let email = Email {
            to: &to,
            from: &from,
            html: "",
            subject: &subject,
            reply_to: "",
            plaintext: &plaintext,
        };

        let res = email_service.send(email);
        assert_ok!(res);

        let sent_messages = email_service.sent_messages.lock().unwrap();
        assert_eq!(1, sent_messages.len());

        if let Some(message) = sent_messages.get(0) {
            assert_eq!(to, message.0);
        } else {
            panic!("Message not sent");
        }
    }
}
