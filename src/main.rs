mod config;
mod db;
mod models;
mod routes;
mod utils;

use axum::{
    Router,
    routing::{get, post, put},
};
use config::Config;
use db::QuoteDatabase;
use routes::{
    quote::get_daily_quote, quote::get_quote_by_id, quote::get_random_quote, quote::update_quote,
    root::root, token::generate_subscription_token,
};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let db = Arc::new(Mutex::new(QuoteDatabase::new(&config.db_path)));
    let pool = SqlitePool::connect_lazy(&config.db_url).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/{id}", put(update_quote))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .with_state(db)
        .route("/admin/token", post(generate_subscription_token))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address {}");

    utils::print_listener_info(&listener);

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
