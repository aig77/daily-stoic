use chrono::{TimeDelta, Utc};
use rust_otp::TOTP;
use sqlx::FromRow;

const OTP_DIGITS_COUNT: u32 = 5;

#[derive(Debug, Clone, FromRow)]
pub struct Otp {
    pub email: String,
    pub code: String,
    pub expires_at: String,
}

impl Otp {
    pub fn new(email: &str) -> Self {
        let t = Utc::now() + TimeDelta::minutes(5);
        Otp {
            email: email.to_string(),
            code: generate_code(),
            expires_at: t.to_rfc3339(),
        }
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
