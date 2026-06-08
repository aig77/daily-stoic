use crate::models::User;
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
        sqlx::query!("INSERT INTO users (email) VALUES (?1)", email,)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn update(&self, email: String, emails_enabled: i64, send_time: String) {
        sqlx::query!(
            "UPDATE users SET emails_enabled = ?1, send_time = ?2 WHERE email = ?3",
            emails_enabled,
            send_time,
            email
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn grant_admin(&self, email: String) {
        sqlx::query!("UPDATE users SET is_admin = 1 WHERE email = ?1", email)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
