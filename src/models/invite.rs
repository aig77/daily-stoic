use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Invite {
    pub id: String,
    pub expires_at: String,
}

impl Default for Invite {
    fn default() -> Self {
        let t = Utc::now() + chrono::Duration::hours(24);
        Self {
            id: Uuid::new_v4().to_string(),
            expires_at: t.to_rfc3339(),
        }
    }
}

impl Invite {
    pub fn is_expired(&self) -> bool {
        let expires_at = self.expires_at.parse::<DateTime<Utc>>().unwrap();
        Utc::now() >= expires_at
    }
}
