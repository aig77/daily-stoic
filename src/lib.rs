pub mod api;
pub mod config;
pub mod database;
pub mod email;
pub mod errors;
pub mod models;
pub mod schedule;

pub use database::Database;
pub use errors::{PageError, ToastError};

use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Budget {
    map: Arc<DashMap<String, (u32, DateTime<Utc>)>>,
    limit: u32,
    window: Duration,
}

impl Budget {
    pub fn new(limit: u32, window: Duration) -> Self {
        Budget {
            map: Arc::new(DashMap::new()),
            limit,
            window,
        }
    }

    pub fn is_limited(&self, email: &str) -> bool {
        let mut entry = self.map.entry(email.to_string()).or_insert((1, Utc::now()));

        let (count, started) = &mut *entry;

        if Utc::now().signed_duration_since(*started) >= self.window {
            *count = 1;
            *started = Utc::now();
        } else if *count > self.limit {
            return true;
        } else {
            *count += 1;
        }

        false
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub db: Database,
    pub daily_sends: Budget,
    pub random_sends: Budget,
    pub schedule_changes: Budget,
}
