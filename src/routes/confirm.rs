//! src/routes/confirm.rs
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;
use tracing::instrument;
use tracing::Instrument;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ConfirmRequest {
    token: String,
}

#[instrument(
    skip(pool),
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
pub async fn confirm(
    info: web::Query<ConfirmRequest>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    info!("Confirming subscription {}", info.token);

    let result = sqlx::query!(
        r#"
        SELECT subscription_token, subscriber_id FROM subscription_tokens
        WHERE subscription_token = $1        
        "#,
        info.token,
    )
    .fetch_one(pool.get_ref())
    .instrument(tracing::info_span!("confirm subscription query"))
    .await;

    match result {
        Ok(record) => {
            info!("Subscription confirmed {}", record.subscriber_id);
            // TODO: if token is in the database, look up user and set status to confirm

            // TODO: delete token from db
        }
        Err(e) => {
            error!("Error fetching confirmation token {}", e);
            // return HttpResponse::BadRequest if token not found
            // return InternalServerError if something else
        }
    }

    HttpResponse::Ok().finish()
}
