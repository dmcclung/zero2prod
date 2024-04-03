//! src/routes/newsletter.rs

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct NewsletterFormData {
    pub newsletter: String,
}

pub async fn publish_newsletter(
    data: web::Form<NewsletterFormData>,
) -> HttpResponse {
    info!("{}", data.newsletter);
    HttpResponse::Ok().finish()
}
