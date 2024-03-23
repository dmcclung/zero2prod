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
            info!("Subscription confirmed {}", record.subscription_token);
            let result = sqlx::query!(
                r#"
                UPDATE subscriptions SET status = 'confirmed' WHERE id = $1
                "#,
                record.subscriber_id
            )
            .execute(pool.get_ref())
            .instrument(tracing::info_span!("set subscription to confirmed"))
            .await;

            match result {
                Ok(_) => {
                    info!("Updated subscription to confirmed");
                    let result = sqlx::query!(
                        r#"
                        DELETE FROM subscription_tokens
                        WHERE subscription_token = $1
                        "#,
                        info.token
                    )
                    .execute(pool.get_ref())
                    .instrument(tracing::info_span!("delete subscription token"))
                    .await;

                    match result {
                        Ok(_) => {
                            info!("Deleted subscription token {}", info.token);
                            HttpResponse::Ok().finish()
                        }
                        Err(e) => {
                            error!("Error deleting subscription token {}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(e) => {
                    error!("Error updating subscription {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            error!("Error fetching confirmation token {}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}
