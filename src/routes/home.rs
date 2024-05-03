//! src/routes/home.rs
use actix_web::HttpResponse;
use askama::Template;

use crate::templates::HomeTemplate;

pub async fn home() -> HttpResponse {
    let home_template = HomeTemplate{};
    let home_rendered = home_template.render().unwrap();
    HttpResponse::Ok().body(home_rendered)
}
