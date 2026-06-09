#[derive(Clone, Debug)]
pub struct Config {
    pub addr: String,
    pub base_url: String,
    pub database_url: String,
    pub resend_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Config {
            addr: std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .expect("Missing required env var DATABASE_URL"),
            resend_api_key: std::env::var("RESEND_API_KEY")
                .expect("Missing required env var RESEND_API_KEY"),
        }
    }
}
