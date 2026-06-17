use chrono::Duration;
use daily_stoic::{
    AppState, Budget,
    api::{configure, middleware::tracing::init_tracing},
    config::Config,
    database::Database,
    email::check_env_vars,
    schedule::init_email_scheduler,
};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    init_tracing();
    info!("Tracing initialized");

    let config = Config::from_env();
    info!(
        "Server configuration retrieved: addr={}, base_url={}, database_url={}",
        &config.addr, &config.base_url, &config.database_url,
    );

    // to ensure email config is setup properly
    check_env_vars();
    info!("Required Resend env vars are configured");

    let db = Database::new(&config.database_url).await;
    info!("Sqlite connection established");

    let state = AppState {
        config: config.clone(),
        db,
        daily_sends: Budget::new(1, Duration::hours(24)),
        random_sends: Budget::new(1, Duration::hours(24)),
        schedule_changes: Budget::new(3, Duration::hours(24)),
    };

    init_email_scheduler(state.clone()).await.unwrap();
    info!("Email scheduler initialized: listening every 15 minutes");

    let app = configure().with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to address {}", &config.addr));

    info!("Server listening at {}", &config.addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Server failed to run");
}
