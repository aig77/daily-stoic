pub mod api;
pub mod config;
pub mod database;
pub mod email;
pub mod models;
pub mod schedule;

pub use database::Database;

use dashmap::DashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub db: Database,
    pub sends: Arc<DashMap<String, (u8, Instant)>>,
}
