use actix_web::{http::header, web, HttpResponse};
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    _username: String,
    _password: Secret<String>,
}

pub async fn login(_form: web::Form<LoginFormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}
