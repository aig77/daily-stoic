use sqlx::FromRow;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Invite {
    pub id: String,
    pub expires_at: String,
}

impl Default for Invite {
    fn default() -> Self {
        let t = OffsetDateTime::now_utc() + Duration::hours(24);
        Self {
            id: Uuid::new_v4().to_string(),
            expires_at: t.format(&Rfc3339).unwrap(),
        }
    }
}

impl Invite {
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() >= OffsetDateTime::parse(&self.expires_at, &Rfc3339).unwrap()
    }
}
