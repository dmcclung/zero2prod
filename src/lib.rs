use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
use chrono::Utc;
use domain::SubscriberName;
use sqlx::{Pool, Postgres};
use tracing::{error, info, instrument, Instrument};
use uuid::Uuid;

pub mod config;
pub mod domain;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize, Debug)]
struct SubscribeData {
    email: String,
    name: SubscriberName,
}

#[instrument(
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(
    form: web::Form<SubscribeData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    info!("Adding a new subscriber");

    info!("Saving new subscriber details in the database");

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name.as_ref(),
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
