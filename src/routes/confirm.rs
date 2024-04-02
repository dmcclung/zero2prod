//! src/routes/confirm.rs
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Postgres;
use tracing::info;
use tracing::instrument;
use tracing::Instrument;
use uuid::Uuid;

use crate::domain::subscriber::SubscriberError;

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
) -> Result<HttpResponse, actix_web::Error> {
    info!("Confirming subscription {}", info.token);

    let subscription_token = sqlx::query!(
        r#"
        SELECT subscription_token, subscriber_id FROM subscription_tokens
        WHERE subscription_token = $1        
        "#,
        info.token,
    )
    .fetch_one(pool.get_ref())
    .instrument(tracing::info_span!("confirm subscription query"))
    .await
    .map_err(|_e| SubscriberError::InvalidToken(info.token.clone()))?;

    info!(
        "Subscription confirmed {}",
        subscription_token.subscription_token
    );
    sqlx::query!(
        r#"
        UPDATE subscriptions SET status = 'confirmed' WHERE id = $1
        "#,
        subscription_token.subscriber_id
    )
    .execute(pool.get_ref())
    .instrument(tracing::info_span!("set subscription to confirmed"))
    .await
    .map_err(SubscriberError::DatabaseError)?;

    info!("Updated subscription to confirmed");
    sqlx::query!(
        r#"
        DELETE FROM subscription_tokens
        WHERE subscription_token = $1
        "#,
        info.token
    )
    .execute(pool.get_ref())
    .instrument(tracing::info_span!("delete subscription token"))
    .await
    .map_err(SubscriberError::DatabaseError)?;

    info!("Deleted subscription token {}", info.token);
    Ok(HttpResponse::Ok().finish())
}
