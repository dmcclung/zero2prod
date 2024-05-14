use actix_web::{http::header, web, HttpResponse};
use secrecy::Secret;
use sqlx::{Pool, Postgres};

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    _username: String,
    _password: Secret<String>,
}

#[tracing::instrument(
    skip(form, pool),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
    )]
pub async fn login(
    form: web::Form<LoginFormData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}
