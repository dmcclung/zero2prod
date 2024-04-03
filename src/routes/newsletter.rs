//! src/routes/newsletter.rs

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct NewsletterJson {
    pub newsletter: String,
}

pub async fn publish_newsletter(
    json: web::Json<NewsletterJson>,
) -> HttpResponse {
    info!("{}", json.0.newsletter);
    HttpResponse::Ok().finish()
}
