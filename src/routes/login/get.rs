use crate::templates::LoginTemplate;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use askama::Template;

pub async fn login_form() -> HttpResponse {
    let login_template = LoginTemplate {};
    let login_rendered = login_template.render().unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(login_rendered);
    HttpResponse::Ok().finish()
}
