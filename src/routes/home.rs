//! src/routes/home.rs
use actix_web::HttpResponse;
use askama::Template;

use crate::templates::HomeTemplate;
use actix_web::http::header::ContentType;

pub async fn home() -> HttpResponse {
    let home_template = HomeTemplate {};
    let home_rendered = home_template.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(home_rendered)
}
