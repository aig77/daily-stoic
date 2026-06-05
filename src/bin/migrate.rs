use daily_stoic::{config::Config, models::Quote};
use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let raw = std::fs::read_to_string(config.db_path).unwrap();
    let quotes: HashMap<String, Quote> = serde_json::from_str(&raw).unwrap();

    let pool = SqlitePool::connect(&config.db_url).await.unwrap();

    for (date, quote) in quotes {
        sqlx::query!(
            "insert into quotes (date, month_topic, season_topic, title, quote, quoter, explanation) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            date,
            quote.title,
            quote.month_topic,
            quote.season_topic,
            quote.quote,
            quote.quoter,
            quote.explanation
        )
        .execute(&pool)
        .await
        .unwrap();
    }
}
