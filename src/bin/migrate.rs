use daily_stoic::{config::Config, models::Quote};
use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let db_path = std::env::var("DATABASE_JSON_PATH")
        .unwrap_or_else(|_| "/var/lib/daily-stoic/database.json".to_string());
    let raw = std::fs::read_to_string(&db_path).unwrap();
    let quotes: HashMap<String, Quote> = serde_json::from_str(&raw).unwrap();

    let pool = SqlitePool::connect(&config.database_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    for (date, quote) in quotes {
        sqlx::query!(
            "insert or ignore into quotes (date, month_topic, season_topic, title, quote, quoter, explanation) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
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

    if let Ok(email) = std::env::var("BOOTSTRAP_ADMIN_EMAIL") {
        sqlx::query!(
            "INSERT OR IGNORE INTO users (email, is_admin) VALUES (?1, 1)",
            email
        )
        .execute(&pool)
        .await
        .unwrap();
    }
}
