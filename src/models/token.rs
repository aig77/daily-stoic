use chrono::{TimeDelta, Utc};
use rand::RngExt;
use sqlx::FromRow;

const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CODE_LEN: usize = 5;

#[derive(Debug, Clone, FromRow)]
pub struct Token {
    pub id: String,
    pub expires_at: String,
}

impl Default for Token {
    fn default() -> Self {
        let t = Utc::now() + TimeDelta::hours(24);
        Self {
            id: generate_random_base62_code(CODE_LEN),
            expires_at: t.to_rfc3339(),
        }
    }
}

fn generate_random_base62_code(length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..62);
            BASE62[idx] as char
        })
        .collect()
}
