use actix_web::{http::header, web, HttpResponse};

#[derive(serde::Deserialize)] 
pub struct LoginFormData {
    _username: String,
    _password: String, 
}

pub async fn login(_form: web::Form<LoginFormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}
