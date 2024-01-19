use actix_web::{web, App, HttpServer, Responder, HttpResponse, dev::Server};

async fn health_check() ->  impl Responder {
    HttpResponse::Ok()
}

pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()            
            .route("/health_check", web::get().to(health_check))
    })
    .bind(address)?
    .run();
    Ok(server)
}   