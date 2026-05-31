use crate::models::SubscriptionToken;
use axum::{extract::State, http::StatusCode};

use sqlx::sqlite::SqlitePool;

pub async fn generate_subscription_token(
    State(pool): State<SqlitePool>,
) -> Result<String, StatusCode> {
    for _ in 0..5 {
        let token = SubscriptionToken::new();
        let count = sqlx::query_scalar!("select count(*) from tokens where id = ?1", token.id)
            .fetch_one(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if count == 0 {
            sqlx::query!(
                "insert into tokens values (?1, ?2);",
                token.id,
                token.created_at
            )
            .execute(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(token.id);
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
