use crate::models::Token;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct TokensRepository {
    pool: SqlitePool,
}

impl TokensRepository {
    pub fn new(pool: SqlitePool) -> Self {
        TokensRepository { pool }
    }

    pub async fn get(&self, id: &str) -> Option<Token> {
        sqlx::query_as!(Token, "SELECT * FROM tokens WHERE id = ?1", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    pub async fn insert(&self, token: &Token) {
        sqlx::query!(
            "INSERT INTO tokens VALUES (?1, ?2)",
            token.id,
            token.expires_at
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
