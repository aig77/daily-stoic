use chrono::{TimeDelta, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Token {
    pub id: String,
    pub expires_at: String,
}

impl Default for Token {
    fn default() -> Self {
        let t = Utc::now() + TimeDelta::hours(24);
        Self {
            id: Uuid::new_v4().to_string(),
            expires_at: t.to_rfc3339(),
        }
    }
}
