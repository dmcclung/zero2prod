//! src/routes/health_check.rs
use actix_web::HttpResponse;

pub async fn confirm() -> HttpResponse {
    HttpResponse::Ok().finish()
}
