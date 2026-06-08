use crate::models::Otp;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct OtpsRepository {
    pool: SqlitePool,
}

impl OtpsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        OtpsRepository { pool }
    }

    pub async fn get(&self, email: &str) -> Option<Otp> {
        sqlx::query_as!(Otp, "SELECT * FROM otps WHERE email = ?1", email)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    pub async fn insert(&self, otp: Otp) {
        sqlx::query!(
            "INSERT INTO otps VALUES (?1, ?2, ?3)",
            otp.email,
            otp.code,
            otp.expires_at
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn delete(&self, email: &str) {
        sqlx::query!("DELETE FROM otps WHERE email = ?1", email)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}
