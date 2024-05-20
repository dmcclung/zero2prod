use crate::templates::LoginTemplate;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use askama::Template;

#[derive(serde::Deserialize, Debug)]
pub struct QueryParams {
    error: Option<String>,
}

#[tracing::instrument()]
pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let _error = query.0.error;
    let login_template = LoginTemplate {};
    let login_rendered = login_template.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(login_rendered);
    HttpResponse::Ok().finish()
}
