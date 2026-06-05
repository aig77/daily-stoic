pub struct Config {
    pub addr: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Config {
            addr: std::env::var("ADDRESS").expect("Missing required env var ADDRESS"),
            database_url: std::env::var("DATABASE_URL")
                .expect("Missing required env var DATABASE_URL"),
        }
    }
}
