use axum::{
    Router,
    response::Redirect,
    routing::{get, post},
};
use daily_stoic::{
    AppState,
    config::Config,
    database::Database,
    middleware::{init_tracing, sessions::create_session_layer},
    routes::{
        invite::generate_invite,
        login::{login_page, resend_otp, submit_login, verify_otp},
        quotes::{get_daily_quote, get_quote_by_id, get_random_quote},
        register::{register_ok_page, register_page, submit_register},
    },
};

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env();
    let db = Database::new(&config.database_url).await;
    let state = AppState {
        config: config.clone(),
        db,
    };
    let session_layer = create_session_layer();

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/login") }))
        .route("/login", get(login_page).post(submit_login))
        .route("/login/verify", post(verify_otp))
        .route("/login/resend", post(resend_otp))
        .route("/invite", post(generate_invite))
        .route("/register/{id}", get(register_page).post(submit_register))
        .route("/register/ok", get(register_ok_page))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .with_state(state)
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address {}");

    println!("Server listening at {}", &config.addr);

    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
