use time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

pub use tower_sessions::Session;

const SESSION_DURATION_SECONDS: i64 = 900;
pub const EMAIL_KEY: &str = "email";

pub fn create_session_layer() -> SessionManagerLayer<MemoryStore> {
    let session_store = MemoryStore::default();
    SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(
            SESSION_DURATION_SECONDS,
        )))
}
