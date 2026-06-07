use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub email: String,
    pub emails_enabled: i64, // db enforces this as a bool 0,1
    pub send_time: String,
}
