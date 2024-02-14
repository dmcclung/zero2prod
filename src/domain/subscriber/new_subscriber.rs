//! new_subscriber.rs

use crate::domain::subscriber::SubscriberName;
use crate::domain::subscriber::SubscriberEmail;

#[derive(serde::Deserialize, Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}