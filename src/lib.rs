pub mod config;
pub mod database;
pub mod handlers;
pub mod middleware;
pub mod models;

pub use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub db: Database,
}
