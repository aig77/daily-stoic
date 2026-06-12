use crate::models::{DateId, Quote};
use sqlx::sqlite::SqlitePool;
use chrono::{Datelike, Utc};

#[derive(Clone, Debug)]
pub struct QuotesRepository {
    pool: SqlitePool,
}

impl QuotesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        QuotesRepository { pool }
    }

    pub async fn get(&self, date_id: &DateId) -> Option<Quote> {
        sqlx::query_as!(
            Quote,
            "SELECT * FROM quotes WHERE date = ?1",
            date_id.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_daily(&self) -> Option<Quote> {
        let now = Utc::now();
        let today = format!("{:02}-{:02}", now.month(), now.day());
        let id = DateId::new(&today).ok()?;
        self.get(&id).await
    }

    pub async fn get_random(&self) -> Option<Quote> {
        sqlx::query_as!(Quote, "SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }
}
