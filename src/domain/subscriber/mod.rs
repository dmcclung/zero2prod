mod error;
mod subscriber_email;
mod subscriber_name;

pub use error::SubscriberError;
pub use subscriber_email::SubscriberEmail;
pub use subscriber_name::SubscriberName;

#[derive(serde::Deserialize, Debug, sqlx::FromRow)]
pub struct Subscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
    pub status: String,
}
