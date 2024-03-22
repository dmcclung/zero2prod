//! src/routes/confirm.rs
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct ConfirmRequest {
    token: String,
}

pub async fn confirm(info: web::Query<ConfirmRequest>) -> HttpResponse {
    // TODO: Get token and lookup from database
    info!("Confirming subscription {}", info.token);
    // if token is in the database, look up user and set status to confirm
    // delete token from db
    HttpResponse::Ok().finish()
}
