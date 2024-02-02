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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
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