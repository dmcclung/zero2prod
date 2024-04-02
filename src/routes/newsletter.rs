//! src/routes/newsletter.rs

use actix_web::HttpResponse;

pub async fn publish_newsletter() -> HttpResponse {
    HttpResponse::Ok().finish()
}
