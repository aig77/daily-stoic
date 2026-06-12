use crate::models::LoginCode;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct LoginCodesRepository {
    pool: SqlitePool,
}

impl LoginCodesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        LoginCodesRepository { pool }
    }

    pub async fn get(&self, email: &str) -> Option<LoginCode> {
        sqlx::query_as!(
            LoginCode,
            "SELECT * FROM login_codes WHERE email = ?1",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn insert(&self, login_code: &LoginCode) {
        sqlx::query!(
            "INSERT INTO login_codes VALUES (?1, ?2, ?3)",
            &login_code.email,
            &login_code.code,
            &login_code.expires_at
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn delete(&self, email: &str) {
        sqlx::query!("DELETE FROM login_codes WHERE email = ?1", email)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
