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

    pub async fn update(&self, email: &str, emails_enabled: i64, send_time: &str, timezone: &str) {
        sqlx::query!(
            "UPDATE users SET emails_enabled = ?1, send_time = ?2, timezone = ?3 WHERE email = ?4",
            emails_enabled,
            send_time,
            timezone,
            email
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn delete(&self, email: &str) {
        sqlx::query!("DELETE FROM users WHERE email = ?1", email)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn get_all_enabled(&self) -> Vec<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE emails_enabled = 1")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn grant_admin(&self, email: &str) {
        sqlx::query!("UPDATE users SET is_admin = 1 WHERE email = ?1", email)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn get_scheduled_users(&self, send_time: &str) -> Vec<String> {
        sqlx::query_scalar!(
            "SELECT email FROM users WHERE emails_enabled = 1 AND send_time = ?1",
            send_time
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
