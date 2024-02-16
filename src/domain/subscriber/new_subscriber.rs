//! new_subscriber.rs

use crate::domain::subscriber::SubscriberEmail;
use crate::domain::subscriber::SubscriberName;

#[derive(serde::Deserialize, Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
