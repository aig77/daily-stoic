use askama::Template;
use axum::{
    body::Body,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use governor::middleware::NoOpMiddleware;
use std::{sync::Arc, time::Duration};
use tower_governor::{
    GovernorError, GovernorLayer, governor::GovernorConfigBuilder,
    key_extractor::PeerIpKeyExtractor,
};

#[derive(Template)]
#[template(path = "errors/rate_limit.html")]
struct RateLimitTemplate {
    message: &'static str,
}

pub fn rate_limiter(
    max: u32,
    window: Duration,
    message: &'static str,
) -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware, Body> {
    let period = window / max;
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .period(period)
            .burst_size(max)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let cleanup_interval = window * 2;

    tokio::spawn(async move {
        let mut interval_timer = tokio::time::interval(cleanup_interval);
        loop {
            interval_timer.tick().await;
            governor_limiter.retain_recent();
        }
    });

    GovernorLayer::new(governor_conf).error_handler(move |error| match error {
        GovernorError::TooManyRequests { .. } => (
            StatusCode::TOO_MANY_REQUESTS,
            [
                ("HX-Retarget", "#toast-container"),
                ("HX-Reswap", "innerHTML"),
            ],
            Html(RateLimitTemplate { message }.render().unwrap()),
        )
            .into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    })
}
