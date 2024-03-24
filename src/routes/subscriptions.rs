use crate::{
    domain::subscriber::{NewSubscriber, SubscriberEmail, SubscriberError, SubscriberName},
    email::{Email, EmailService},
};
use actix_web::{web, HttpResponse};
use anyhow::Result;
use askama::Template;
use chrono::Utc;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::{fmt::Debug, sync::Arc};
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
) -> HttpResponse {
    info!("Adding a new subscriber");

    let new_subscriber = match parse_subscriber(data.0) {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let result = sqlx::query!(
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
    .await;

    match result {
        Ok(sub_record) => {
            info!("New subscriber details has been saved");

            let subscription_token = Uuid::new_v4().to_string();

            let result = sqlx::query!(
                r#"
                INSERT INTO subscription_tokens (subscription_token, subscriber_id)
                VALUES ($1, $2)
                "#,
                subscription_token,
                sub_record.id
            )
            .execute(pool.get_ref())
            .instrument(tracing::info_span!("add subscription token query"))
            .await;

            match result {
                Ok(_) => {
                    match send_confirmation_email(
                        &sub_record.email,
                        &subscription_token,
                        email_service,
                    ) {
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
                    error!("Failed to insert subscription token query: {:?}", e);
                    println!("{}", e);
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

#[derive(Template)]
#[template(path = "confirmation/email.html")]
struct ConfirmationEmailHtmlTemplate<'a> {
    token: &'a str,
}

#[derive(Template)]
#[template(path = "confirmation/email.txt")]
struct ConfirmationEmailTxtTemplate<'a> {
    token: &'a str,
}

#[derive(Template)]
#[template(path = "confirmation/subject.txt")]
struct ConfirmationEmailSubject {}

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
