mod config;
mod database;
mod models;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use config::Config;
use database::Database;
use routes::{
    quote::get_daily_quote, quote::get_quote_by_id, quote::get_random_quote, root::root,
    token::generate_token,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let db = Database::new(&config.database_url);

    let app = Router::new()
        .route("/", get(root))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .route("/token", post(generate_token))
        .with_state(db.await);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address {}");

    println!("Server listening at {}", &config.addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
