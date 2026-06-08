pub mod config;
pub mod database;
pub mod middleware;
pub mod models;
pub mod routes;

pub use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub db: Database,
}
