use chrono::{DateTime, Utc};
use sqlx::FromRow;

const CODE_LENGTH: u32 = 5;

#[derive(Debug, Clone, FromRow)]
pub struct LoginCode {
    pub email: String,
    pub code: String,
    pub expires_at: String,
}

impl LoginCode {
    pub fn new(email: &str) -> Self {
        let t = Utc::now() + chrono::Duration::minutes(5);
        Self {
            email: email.to_string(),
            code: generate_code(),
            expires_at: t.to_rfc3339(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let expires_at = self.expires_at.parse::<DateTime<Utc>>().unwrap();
        Utc::now() >= expires_at
    }
}

fn generate_code() -> String {
    let min = 10u32.pow(CODE_LENGTH - 1);
    let max = 10u32.pow(CODE_LENGTH);
    (rand::random::<u32>() % (max - min) + min).to_string()
}
