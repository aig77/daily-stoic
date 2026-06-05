pub struct Config {
    pub addr: String,
    pub db_path: String,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Config {
            addr: std::env::var("ADDRESS").expect("Missing required env var ADDRESS"),
            db_path: std::env::var("DATABASE_PATH")
                .expect("Missing required env var DATABASE_PATH"),
            db_url: std::env::var("DATABASE_URL").expect("Missing required env var DATABASE_URL"),
        }
    }
}
