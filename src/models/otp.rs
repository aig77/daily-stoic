use chrono::{TimeDelta, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Otp {
    pub email: String,
    pub code: String,
    pub expires_at: String,
}

impl Otp {
    pub fn new(email: &str, code: &str) -> Self {
        let t = Utc::now() + TimeDelta::minutes(5);
        Otp {
            email: email.to_string(),
            code: code.to_string(),
            expires_at: t.to_string(),
        }
    }
}
