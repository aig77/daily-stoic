use crate::models::{DateId, Quote};
use chrono::{Datelike, Utc};
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct QuotesRepository {
    pool: SqlitePool,
}

impl QuotesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        QuotesRepository { pool }
    }

    pub async fn get(&self, date_id: &DateId) -> Result<Option<Quote>, sqlx::Error> {
        sqlx::query_as!(
            Quote,
            "SELECT * FROM quotes WHERE date = ?1",
            date_id.as_str()
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_daily(&self) -> Result<Option<Quote>, sqlx::Error> {
        let now = Utc::now();
        let today = format!("{:02}-{:02}", now.month(), now.day());
        let Ok(id) = DateId::new(&today) else {
            return Ok(None);
        };
        self.get(&id).await
    }

    pub async fn get_random(&self) -> Result<Option<Quote>, sqlx::Error> {
        sqlx::query_as!(Quote, "SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1")
            .fetch_optional(&self.pool)
            .await
    }
}
