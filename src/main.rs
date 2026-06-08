mod config;
mod database;
mod middleware;
mod models;
mod routes;

use axum::{
    Router,
    response::Redirect,
    routing::{get, post},
};
use config::Config;
use database::Database;
use middleware::{init_tracing, sessions::create_session_layer};
use routes::{
    login::{login_page, resend_otp, submit_login, verify_otp},
    quotes::{get_daily_quote, get_quote_by_id, get_random_quote},
    register::{register_page, registered_page, submit_register},
    tokens::generate_token,
};

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env();

    let db = Database::new(&config.database_url).await;
    let session_layer = create_session_layer();

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/login") }))
        .route("/login", get(login_page).post(submit_login))
        .route("/verify", post(verify_otp))
        .route("/register/{id}", get(register_page).post(submit_register))
        .route("/registered", get(registered_page))
        .route("/resend", post(resend_otp))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .route("/token", post(generate_token))
        .with_state(db)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address {}");

    println!("Server listening at {}", &config.addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
