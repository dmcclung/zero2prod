use actix_web::{http::header, HttpResponse};

pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}
