mod subscriber_email;
mod subscriber_error;
mod subscriber_name;

pub use subscriber_email::SubscriberEmail;
pub use subscriber_error::SubscriberError;
pub use subscriber_name::SubscriberName;

#[derive(serde::Deserialize, Debug)]
pub struct Subscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
    pub status: String,
}
