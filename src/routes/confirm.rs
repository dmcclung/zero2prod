//! src/routes/health_check.rs
use actix_web::HttpResponse;

pub async fn confirm() -> HttpResponse {
    // TODO: Get token and lookup from database
    // if token is in the database, look up user and set status to confirm
    // delete token from db
    HttpResponse::Ok().finish()
}
