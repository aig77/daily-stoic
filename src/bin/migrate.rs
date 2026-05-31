use daily_stoic::models::Quote;
use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let raw = std::fs::read_to_string("database.json").unwrap();
    let quotes: HashMap<String, Quote> = serde_json::from_str(&raw).unwrap();

    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

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
