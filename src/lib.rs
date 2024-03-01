use actix_web::{web, HttpResponse};
use chrono::Utc;
use domain::subscriber::{NewSubscriber, SubscriberEmail, SubscriberError, SubscriberName};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::{error, info, instrument, Instrument};
use uuid::Uuid;
use anyhow::Result;

pub mod config;
pub mod domain;
pub mod email;
pub mod app;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SubscriberFormData {
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
async fn subscribe(
    data: web::Form<SubscriberFormData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    info!("Adding a new subscriber");

    let new_subscriber = match parse_subscriber(data.0) {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
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
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
