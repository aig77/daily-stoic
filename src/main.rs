mod config;
mod database;
mod models;
mod routes;

use axum::{
    Router,
    response::Redirect,
    routing::{get, post},
};
use config::Config;
use database::Database;
use routes::{
    login::{login_page, submit_login},
    quotes::{get_daily_quote, get_quote_by_id, get_random_quote},
    register::{register_page, submit_register},
    tokens::generate_token,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let db = Database::new(&config.database_url).await;

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/login") }))
        .route("/login", get(login_page))
        .route("/login", post(submit_login))
        .route("/register", get(register_page))
        .route("/register", post(submit_register))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .route("/token", post(generate_token))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address {}");

    println!("Server listening at {}", &config.addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
