use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use domain::NewSubscriber;
use sqlx::{Pool, Postgres};
use tracing::{error, info, instrument, Instrument};
use uuid::Uuid;

pub mod config;
pub mod domain;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[instrument(
    skip(new_subscriber, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %new_subscriber.email,
        subscriber_name = %new_subscriber.name
    )
)]
async fn subscribe(
    new_subscriber: web::Form<NewSubscriber>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    info!("Adding a new subscriber");

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
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
