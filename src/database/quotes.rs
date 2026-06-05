use crate::models::{DateId, Quote};
use chrono::Local;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct QuotesRepository {
    pool: SqlitePool,
}

impl QuotesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        QuotesRepository { pool }
    }

    pub async fn get(&self, date_id: DateId) -> Option<Quote> {
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
        // TODO: make this work with any timezone the request is in
        let today = Local::now().format("%m-%d").to_string();
        let id = DateId::new(&today).ok()?;
        self.get(id).await
    }

    pub async fn get_random(&self) -> Option<Quote> {
        sqlx::query_as!(Quote, "SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }
}
