use std::sync::Arc;

use daily_stoic::{
    AppState,
    api::{configure, middleware::tracing::init_tracing},
    config::Config,
    database::Database,
};
use dashmap::DashMap;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();
    info!("Tracing initialized");

    let config = Config::from_env();
    info!(
        "Server configuration retrieved: addr={}, base_url={}, database_url={}, resend_api_key(LEN)={}",
        &config.addr,
        &config.base_url,
        &config.database_url,
        &config.resend_api_key.len(),
    );

    let db = Database::new(&config.database_url).await;
    info!("Sqlite connection established");

    let state = AppState {
        config: config.clone(),
        db,
        sends: Arc::new(DashMap::new()),
    };

    let app = configure().with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address {}");

    info!("Server listening at {}", &config.addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Server failed to run");
}
