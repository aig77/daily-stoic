use crate::Database;
use crate::models::Token;
use axum::{extract::State, http::StatusCode};

pub async fn generate_token(State(db): State<Database>) -> Result<String, StatusCode> {
    for _ in 0..5 {
        let token = Token::default();

        if db.tokens.get(&token.id).await.is_none() {
            db.tokens.insert(&token).await;
            return Ok(token.id);
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
