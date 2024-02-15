use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use domain::subscriber::{ NewSubscriber, SubscriberEmail, SubscriberName };
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tracing::{error, info, instrument, Instrument};
use uuid::Uuid;

pub mod config;
pub mod domain;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SubscriberFormData {
    pub email: String,
    pub name: String,
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

    let email = match SubscriberEmail::parse(data.0.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let name = match SubscriberName::parse(data.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let new_subscriber = NewSubscriber {
        email,
        name
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

pub fn run(
    listener: TcpListener,
    pool: Pool<Postgres>,
) -> Result<Server, Box<dyn std::error::Error>> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        let pool = pool.clone();

        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
