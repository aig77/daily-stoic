#[derive(Clone, Debug)]
pub struct Config {
    pub addr: String,
    pub base_url: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Config {
            addr: std::env::var("ADDRESS").expect("Missing required env var ADDRESS"),
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .expect("Missing required env var DATABASE_URL"),
        }
    }
}
