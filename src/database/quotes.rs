use crate::models::{DateId, Quote};
use sqlx::sqlite::SqlitePool;
use time::OffsetDateTime;

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
        // TODO: make this work with any timezone the request is in
        let now = OffsetDateTime::now_local().unwrap();
        let today = format!("{:02}-{:02}", now.month() as u8, now.day());
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
