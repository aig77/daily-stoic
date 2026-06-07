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

    pub async fn update(&self, emails_enabled: i64, send_time: String) {
        sqlx::query!(
            "UPDATE users SET emails_enabled = ?1, send_time = ?2",
            emails_enabled,
            send_time
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
