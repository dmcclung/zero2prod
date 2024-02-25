//! src/email.rs

use std::env;

use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use anyhow::Result;

use tracing::info;

const DEFAULT_SENDER: &str = "admin@zero2prod.xyz";

pub struct SmtpConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl SmtpConfig {
    pub fn parse_from_env() -> SmtpConfig {
        dotenv::dotenv().ok();

        let host = env::var("EMAIL_HOST").unwrap_or("localhost".into());
        let port = env::var("EMAIL_PORT").unwrap_or("528".into());
        let user = env::var("EMAIL_USER").unwrap_or("user".into());
        let password = env::var("EMAIL_PASSWORD").unwrap_or("password".into());

        SmtpConfig {
            host,
            port: port.parse::<u16>().unwrap(),
            user,
            password,
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
///     to: "recipient@example.com".to_string(),
///     from: "sender@example.com".to_string(),
///     subject: "Greetings!".to_string(),
///     reply_to: "no-reply@example.com".to_string(),
///     html: "<h1>Hello</h1><p>How are you?</p>".to_string(),
///     plaintext: "Hello\nHow are you?".to_string(),
/// };
/// ```
#[derive(Debug)]
pub struct Email {
    /// The recipient's email address.
    pub to: String,
    /// The HTML content of the email message. This field allows the inclusion of
    /// HTML tags for formatting purposes.
    pub html: String,
    /// The sender's email address.
    pub from: String,
    /// The subject of the email message.
    pub subject: String,
    /// The email address for reply-to field, which indicates where replies to the
    /// email should be sent.
    pub reply_to: String,
    /// The plaintext content of the email message. This field is used for email
    /// clients that do not support HTML content or as a fallback.
    pub plaintext: String,
}

pub struct EmailService {
    config: SmtpConfig,
}

impl EmailService {
    pub fn new(config: SmtpConfig) -> EmailService {
        EmailService { config }
    }

    pub fn send_email(&self, email: Email) -> Result<()> {
        let to: Mailbox = email.to.parse()?;
        let from: Mailbox = if email.from.is_empty() {
            DEFAULT_SENDER.parse()?
        } else {
            email.from.parse()?
        };

        let mut message_builder = Message::builder().from(from).to(to).subject(email.subject);

        if !email.reply_to.is_empty() {
            let reply_to: Mailbox = email.reply_to.parse()?;

            message_builder = message_builder.reply_to(reply_to);
        }

        let message = message_builder
            .header(if !email.html.is_empty() {
                ContentType::TEXT_HTML
            } else {
                ContentType::TEXT_PLAIN
            })
            .body(if !email.html.is_empty() {
                email.html
            } else {
                email.plaintext
            })?;

        let creds = Credentials::new(self.config.user.clone(), self.config.password.clone());

        let mailer = SmtpTransport::relay(&self.config.host)?
            .port(self.config.port)
            .credentials(creds)
            .build();

        mailer
            .send(&message)
            .map(|_| info!("Email sent successfully"))
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use std::env::{remove_var, set_var};
    use fake::faker::internet::en::{Username, DomainSuffix, Password};
    use fake::faker::number::en::NumberWithFormat;
    use fake::Fake;

    use super::SmtpConfig;

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
        assert_eq!(hostname, smtp_config.host);
        assert_eq!(username, smtp_config.user);
        assert_eq!(port.parse::<u16>().unwrap(), smtp_config.port);
        assert_eq!(password, smtp_config.password);
        
        remove_var("EMAIL_HOST");
        remove_var("EMAIL_USER");
        remove_var("EMAIL_PORT");
        remove_var("EMAIL_PASSWORD");
    }

    fn generate_hostname() -> String {
        let domain: String = Username().fake();
        let domain_suffix: String = DomainSuffix().fake();
        format!("smtp.{}.{}", domain, domain_suffix)
    }
}
