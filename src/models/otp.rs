use rust_otp::TOTP;
use sqlx::FromRow;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

const OTP_DIGITS_COUNT: u32 = 8;

#[derive(Debug, Clone, FromRow)]
pub struct Otp {
    pub email: String,
    pub code: String,
    pub expires_at: String,
}

impl Otp {
    pub fn new(email: &str) -> Self {
        let t = OffsetDateTime::now_utc() + Duration::minutes(5);
        Otp {
            email: email.to_string(),
            code: generate_code(),
            expires_at: t.format(&Rfc3339).unwrap(),
        }
    }

    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() >= OffsetDateTime::parse(&self.expires_at, &Rfc3339).unwrap()
    }
}

pub fn generate_code() -> String {
    let totp = TOTP::builder()
        .base32_secret("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ")
        .unwrap()
        .digits(OTP_DIGITS_COUNT)
        .build()
        .unwrap();

    totp.generate_current_formatted().unwrap()
}
