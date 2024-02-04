use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;
use tracing::Instrument;

pub mod config;

async fn health_check() ->  HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscribeData {
    email: String,
    name: String
}

async fn subscribe(form: web::Form<SubscribeData>, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    
    let _request_span_guard = request_span.enter();
    
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database",
        %request_id
    );

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await;

    match result {
        Ok(_) => {
            tracing::info!("New subscriber details has been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }    
}
    
pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> Result<Server, Box<dyn std::error::Error>> {
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