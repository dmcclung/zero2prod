//! src/email.rs

use std::env;

use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use anyhow::Result;

use tracing::info;

pub struct SmtpConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    default_sender: String,
}

impl SmtpConfig {
    pub fn new(
        host: String,
        port: String,
        user: String,
        password: String,
        default_sender: String,
    ) -> Self {
        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            user,
            password,
            default_sender,
        }
    }

    pub fn parse_from_env() -> Self {
        dotenv::dotenv().ok();

        let host = env::var("EMAIL_HOST").unwrap();
        let port = env::var("EMAIL_PORT").unwrap();
        let user = env::var("EMAIL_USER").unwrap();
        let password = env::var("EMAIL_PASSWORD").unwrap();
        let default_sender = env::var("EMAIL_DEFAULT_SENDER").unwrap();

        Self {
            host,
            port: port.parse::<u16>().unwrap(),
            user,
            password,
            default_sender,
        }
    }
}

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
#[derive(Debug)]
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

pub trait EmailSender {
    fn send(&mut self, port: u16, host: &str, creds: Credentials, message: Message) -> Result<()>;
}

struct LettreEmailSender;

impl EmailSender for LettreEmailSender {
    fn send(&mut self, port: u16, host: &str, creds: Credentials, message: Message) -> Result<()> {
        let mailer = SmtpTransport::relay(host)?
            .port(port)
            .credentials(creds)
            .build();

        mailer
            .send(&message)
            .map(|_| info!("Email sent successfully"))
            .map_err(|e| e.into())
    }
}

pub struct EmailService<'a, T: EmailSender> {
    config: SmtpConfig,
    email_sender: &'a mut T,
}

impl<'a, T: EmailSender> EmailService<'a, T> {
    pub fn new(config: SmtpConfig, email_sender: &'a mut T) -> Self {
        Self {
            config,
            email_sender,
        }
    }

    pub fn send_email(&mut self, email: Email) -> Result<()> {
        let to: Mailbox = email.to.parse()?;
        let from: Mailbox = if email.from.is_empty() {
            self.config.default_sender.parse()?
        } else {
            email.from.parse()?
        };

        let mut message_builder = Message::builder().from(from).to(to).subject(email.subject);

        if !email.reply_to.is_empty() {
            let reply_to: Mailbox = email.reply_to.parse()?;

            message_builder = message_builder.reply_to(reply_to);
        }

        let message = message_builder
            .header(if !String::from(email.html).is_empty() {
                ContentType::TEXT_HTML
            } else {
                ContentType::TEXT_PLAIN
            })
            .body(if !String::from(email.html).is_empty() {
                String::from(email.html)
            } else {
                String::from(email.plaintext)
            })?;

        let creds = Credentials::new(self.config.user.clone(), self.config.password.clone());
        self.email_sender
            .send(self.config.port, &self.config.host, creds, message)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use claims::assert_ok;
    use fake::faker::internet::en::{DomainSuffix, Password, SafeEmail, Username};
    use fake::faker::lorem::en::Sentence;
    use fake::faker::number::en::NumberWithFormat;
    use fake::Fake;
    use lettre::Message;
    use std::env::{remove_var, set_var};

    use super::{Email, EmailSender, EmailService, SmtpConfig};

    #[test]
    fn smtp_config_from_env() {
        let hostname = generate_hostname();
        let username: String = Username().fake();
        let port: String = NumberWithFormat("###").fake();
        let password: String = Password(8..16).fake();

        set_var("EMAIL_HOST", hostname.clone());
        set_var("EMAIL_USER", username.clone());
        set_var("EMAIL_PORT", port.clone());
        set_var("EMAIL_PASSWORD", password.clone());

        let smtp_config = SmtpConfig::parse_from_env();

        remove_var("EMAIL_HOST");
        remove_var("EMAIL_USER");
        remove_var("EMAIL_PORT");
        remove_var("EMAIL_PASSWORD");

        assert_eq!(hostname, smtp_config.host);
        assert_eq!(username, smtp_config.user);
        assert_eq!(port.parse::<u16>().unwrap(), smtp_config.port);
        assert_eq!(password, smtp_config.password);
    }

    fn generate_hostname() -> String {
        let domain: String = Username().fake();
        let domain_suffix: String = DomainSuffix().fake();
        format!("smtp.{}.{}", domain, domain_suffix)
    }

    struct MockEmailSender {
        sent_messages: Vec<Message>,
    }

    impl MockEmailSender {
        fn new() -> Self {
            Self {
                sent_messages: Vec::new(),
            }
        }
    }

    impl EmailSender for MockEmailSender {
        fn send(
            &mut self,
            _port: u16,
            _host: &str,
            _creds: lettre::transport::smtp::authentication::Credentials,
            message: lettre::Message,
        ) -> Result<()> {
            self.sent_messages.push(message);
            Ok(())
        }
    }

    #[test]
    fn send_valid_email() {
        let smtp_config = SmtpConfig::new(
            generate_hostname(),
            NumberWithFormat("###").fake(),
            Username().fake(),
            Password(8..16).fake(),
            SafeEmail().fake(),
        );
        let email_sender = &mut MockEmailSender::new();

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

        let mut email_service: EmailService<MockEmailSender> =
            EmailService::new(smtp_config, email_sender);
        let res = email_service.send_email(email);

        assert_ok!(res);
        assert_eq!(1, email_sender.sent_messages.len());
    }
}
