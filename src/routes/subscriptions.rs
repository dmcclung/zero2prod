use crate::{
    domain::subscriber::{NewSubscriber, SubscriberEmail, SubscriberError, SubscriberName},
    email::{Email, EmailSender, EmailService},
};
use actix_web::{web, HttpResponse};
use anyhow::Result;
use chrono::Utc;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::fmt::Debug;
use std::sync::Mutex;
use tracing::{error, info, instrument, Instrument};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscriberFormData {
    pub email: String,
    pub name: String,
}

fn parse_subscriber(data: SubscriberFormData) -> Result<NewSubscriber, SubscriberError> {
    let email = SubscriberEmail::parse(data.email)?;
    let name = SubscriberName::parse(data.name)?;

    let new_subscriber = NewSubscriber { email, name };
    Ok(new_subscriber)
}

#[instrument(
    skip(data, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %data.email,
        subscriber_name = %data.name
    )
)]
pub async fn subscribe<'a, T: EmailSender + Debug>(
    data: web::Form<SubscriberFormData>,
    pool: web::Data<Pool<Postgres>>,
    email_service: web::Data<Mutex<EmailService<'a, T>>>,
) -> HttpResponse {
    info!("Adding a new subscriber");

    let new_subscriber = match parse_subscriber(data.0) {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(tracing::info_span!("add subscriber query"))
    .await;

    match result {
        Ok(_) => {
            info!("New subscriber details has been saved");
            match send_confirmation_email(new_subscriber.email.as_ref(), email_service) {
                Ok(_) => {
                    info!("Email sent");
                    HttpResponse::Ok().finish()
                }
                Err(e) => {
                    error!("Failed to send email: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn send_confirmation_email<T: EmailSender>(
    new_subscriber_email: &str,
    email_service: web::Data<Mutex<EmailService<'_, T>>>,
) -> Result<()> {
    let email = Email {
        to: new_subscriber_email,
        from: "",
        subject: "Welcome to zero2prod.xyz",
        reply_to: "",
        plaintext: "We're glad you're here, confirm your subscription https://zero2prod.xyz/confirm?token=a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0",
        html: "",
    };
    email_service.lock().unwrap().send_email(email)
}
