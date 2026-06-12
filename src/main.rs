use std::sync::Arc;

use daily_stoic::{
    AppState,
    api::{configure, middleware::tracing::init_tracing},
    config::Config,
    database::Database,
    email::check_env_vars,
    schedule::init_email_scheduler,
};
use dashmap::DashMap;
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

    init_email_scheduler(db.clone()).await.unwrap();
    info!("Email scheduler initialized: listening every 15 minutes");

    let state = AppState {
        config: config.clone(),
        db,
        sends: Arc::new(DashMap::new()),
    };

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
