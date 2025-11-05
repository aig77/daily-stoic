mod db;
mod models;
mod routes;

use axum::{
    Router,
    routing::{get, put},
};
use db::QuoteDatabase;
use routes::{
    quote::get_daily_quote, quote::get_quote_by_id, quote::get_random_quote, quote::update_quote,
    root::root,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let addr = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "database.json".to_string());

    let db = Arc::new(Mutex::new(QuoteDatabase::new(&db_path)));

    let app = Router::new()
        .route("/", get(root))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/{id}", put(update_quote))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address {}");

    print_listener_info(&listener);

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}

fn print_listener_info(listener: &tokio::net::TcpListener) {
    let routes = vec![
        "🟢 GET     /",
        "",
        "🟢 GET     /quote/{id}",
        "🟡 PUT     /quote/{id}",
        "",
        "🟢 GET     /quote/daily",
        "🟢 GET     /quote/random",
    ];

    let listener_addr = listener
        .local_addr()
        .expect("Failed to get listener address");

    println!("listening on http://{}", listener_addr);

    for route in routes {
        println!("{route}");
    }
}
