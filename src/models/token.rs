use chrono::Utc;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Token {
    pub id: String,
    pub created_at: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().to_string(),
        }
    }
}
