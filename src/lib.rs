use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;

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
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );

    tracing::info!(
        "request_id {} - Saving new subscriber details",
        request_id
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
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details has been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::info!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
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