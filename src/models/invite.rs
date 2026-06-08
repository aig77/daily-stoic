use rand::RngExt;
use sqlx::FromRow;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CODE_LEN: usize = 8;

#[derive(Debug, Clone, FromRow)]
pub struct Invite {
    pub id: String,
    pub expires_at: String,
}

impl Default for Invite {
    fn default() -> Self {
        let t = OffsetDateTime::now_utc() + Duration::hours(24);
        Self {
            id: generate_random_base62_code(CODE_LEN),
            expires_at: t.format(&Rfc3339).unwrap(),
        }
    }
}

impl Invite {
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() >= OffsetDateTime::parse(&self.expires_at, &Rfc3339).unwrap()
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
