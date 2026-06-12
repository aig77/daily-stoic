pub mod handlers;
pub mod middleware;

use crate::AppState;

use axum::{
    Router,
    response::Redirect,
    routing::{get, post},
};
use handlers::{
    login::{login_page, resend_login_code, session_expired_page, submit_login, verify_login_code},
    quotes::{get_daily_quote, get_quote_by_id, get_random_quote},
    register::{register_ok_page, register_page, submit_register},
    settings::{
        delete_confirm_form, delete_user, generate_invite_link, save_settings, send_daily,
        send_random, settings_page,
    },
};
use middleware::{rate_limit::rate_limiter, sessions::create_session_layer};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

const MINUTE: Duration = Duration::from_secs(60);
const HOUR: Duration = Duration::from_secs(3600);

pub fn configure() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { Redirect::temporary("/login") }))
        .route("/login", get(login_page).post(submit_login))
        .route("/session-expired", get(session_expired_page))
        .route(
            "/login/verify",
            post(verify_login_code).route_layer(rate_limiter(10, HOUR, "Too many attempts. Please try again later.")),
        )
        .route(
            "/login/resend",
            post(resend_login_code).route_layer(rate_limiter(3, HOUR, "You can only resend 3 times per hour. Please try again later.")),
        )
        .route("/register/{id}", get(register_page).post(submit_register))
        .route("/register/ok", get(register_ok_page))
        .route("/settings", get(settings_page).post(save_settings))
        .route("/settings/send/daily", post(send_daily))
        .route("/settings/send/random", post(send_random))
        .route("/settings/delete", post(delete_user))
        .route("/settings/delete-confirm", get(delete_confirm_form))
        .route("/settings/invite", post(generate_invite_link))
        .route("/quote/{id}", get(get_quote_by_id))
        .route("/quote/daily", get(get_daily_quote))
        .route("/quote/random", get(get_random_quote))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(create_session_layer())
                .layer(rate_limiter(10, MINUTE, "Too many requests!")),
        )
}
