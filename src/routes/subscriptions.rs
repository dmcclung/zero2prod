use crate::{
    domain::subscriber::{Subscriber, SubscriberEmail, SubscriberError, SubscriberName},
    email::{Email, EmailService},
    templates::{
        ConfirmationEmailHtmlTemplate, ConfirmationEmailSubject, ConfirmationEmailTxtTemplate,
    },
};
use actix_web::{web, HttpResponse};
use askama::Template;
use chrono::Utc;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::{fmt::Debug, sync::Arc};
use tracing::{info, instrument, Instrument};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubscriberFormData {
    pub email: String,
    pub name: String,
}

fn parse_subscriber(data: SubscriberFormData) -> Result<Subscriber, SubscriberError> {
    let email = SubscriberEmail::parse(data.email)?;
    let name = SubscriberName::parse(data.name)?;

    let new_subscriber = Subscriber {
        email,
        name,
        status: "pending".to_string(),
    };
    Ok(new_subscriber)
}

#[instrument(
    skip(data, pool, email_service),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %data.email,
        subscriber_name = %data.name
    )
)]
pub async fn subscribe(
    data: web::Form<SubscriberFormData>,
    pool: web::Data<Pool<Postgres>>,
    email_service: web::Data<Arc<dyn EmailService + Send + Sync>>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("Adding a new subscriber");

    let new_subscriber = parse_subscriber(data.0)?;

    let subscription_record = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending')
        RETURNING id, email, name, subscribed_at, status
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .fetch_one(pool.get_ref())
    .instrument(tracing::info_span!("add subscriber query"))
    .await
    .map_err(SubscriberError::DatabaseError)?;

    info!("New subscriber details has been saved");

    let subscription_token = Uuid::new_v4().to_string();

    sqlx::query!(
        r#"
        INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)
        "#,
        subscription_token,
        subscription_record.id
    )
    .execute(pool.get_ref())
    .instrument(tracing::info_span!("add subscription token query"))
    .await
    .map_err(SubscriberError::DatabaseError)?;

    send_confirmation_email(
        &subscription_record.email,
        &subscription_token,
        email_service,
    )
    .map_err(SubscriberError::EmailError)?;

    info!("Email sent");
    Ok(HttpResponse::Ok().finish())
}

fn send_confirmation_email(
    new_subscriber_email: &str,
    token: &str,
    email_service: web::Data<Arc<dyn EmailService + Send + Sync>>,
) -> Result<(), String> {
    let confirm_email_html = ConfirmationEmailHtmlTemplate { token };
    let confirm_email_plaintext = ConfirmationEmailTxtTemplate { token };
    let confirm_subject = ConfirmationEmailSubject {};

    let email = Email {
        to: new_subscriber_email,
        from: "",
        subject: &confirm_subject.render().unwrap(),
        reply_to: "",
        plaintext: &confirm_email_plaintext.render().unwrap(),
        html: &confirm_email_html.render().unwrap(),
    };
    email_service.send(email)
}
