use std::net::TcpListener;

use actix_web::{web, App, HttpServer, HttpResponse, dev::Server};

async fn health_check() ->  HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscribeData {
    email: String,
    name: String
}

async fn subscribe(_form: web::Form<SubscribeData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, Box<dyn std::error::Error>> {
    let server = HttpServer::new(|| {
        App::new()            
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}   