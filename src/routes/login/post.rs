use actix_web::{http::header, web, HttpResponse};
use secrecy::Secret;
use sqlx::{Pool, Postgres};

use crate::auth::{validate_credentials, Credentials};

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(form, pool),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
    )]
pub async fn login(
    form: web::Form<LoginFormData>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, actix_web::Error> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    let user_id = validate_credentials(credentials, pool.get_ref()).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(user_id));

    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish())
}
