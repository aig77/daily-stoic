use crate::models::User;
use chrono::Utc;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct UsersRepository {
    pool: SqlitePool,
}

impl UsersRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, email: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?1", email)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    pub async fn insert(&self, email: &str) {
        sqlx::query!(
            "INSERT INTO users (email, created_at) VALUES (?1, ?2)",
            email,
            Utc::now().to_string()
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
