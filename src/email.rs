//! src/email.rs

use std::env;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::Mailbox;

use anyhow::Result;

use tracing::info;

pub struct SmtpConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl SmtpConfig {
    pub fn parse_from_env() -> Result<SmtpConfig> {
        dotenv::dotenv().ok();

        let host = env::var("EMAIL_HOST")?;
        
        let port = env::var("EMAIL_PORT")?;
        let user = env::var("EMAIL_USER")?;
        let password = env::var("EMAIL_PASSWORD")?;

        let smtp_config = SmtpConfig {
            host,
            port: port.parse::<u16>()?,
            user,
            password
        };

        Ok(smtp_config)
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
    config: SmtpConfig
}

impl EmailService {
    pub fn new(config: SmtpConfig) -> EmailService {
        EmailService {
            config
        }
    }

    pub fn send_email(&self, email: Email) -> Result<()> {
        let to: Mailbox = email.to.parse()?;
        let from: Mailbox = email.from.parse()?;
        
        let mut message_builder = Message::builder()
            .from(from)
            .to(to)
            .subject(email.subject);            

        if !email.reply_to.is_empty() {
            let reply_to: Mailbox = email.reply_to.parse()?;                

            message_builder = message_builder.reply_to(reply_to);
        }

        let message = message_builder
            .header(if !email.html.is_empty() { ContentType::TEXT_HTML } else { ContentType::TEXT_PLAIN })
            .body(if !email.html.is_empty() { email.html } else { email.plaintext })?;
            
        
        let creds = Credentials::new(self.config.user.clone(), self.config.password.clone());
                
        let mailer = SmtpTransport::relay(&self.config.host)?
            .port(self.config.port)
            .credentials(creds)
            .build();

        mailer.send(&message)
            .map(|_| info!("Email sent successfully"))
            .map_err(|e| e.into())
    }
}