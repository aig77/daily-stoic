use sqlx::FromRow;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

const CODE_LENGTH: u32 = 5;

#[derive(Debug, Clone, FromRow)]
pub struct LoginCode {
    pub email: String,
    pub code: String,
    pub expires_at: String,
}

impl LoginCode {
    pub fn new(email: &str) -> Self {
        let t = OffsetDateTime::now_utc() + Duration::minutes(5);
        Self {
            email: email.to_string(),
            code: generate_code(),
            expires_at: t.format(&Rfc3339).unwrap(),
        }
    }

    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() >= OffsetDateTime::parse(&self.expires_at, &Rfc3339).unwrap()
    }
}

fn generate_code() -> String {
    let min = 10u32.pow(CODE_LENGTH - 1);
    let max = 10u32.pow(CODE_LENGTH);
    (rand::random::<u32>() % (max - min) + min).to_string()
}
