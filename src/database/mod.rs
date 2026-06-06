mod quotes;
mod tokens;
mod users;

use quotes::QuotesRepository;
use sqlx::sqlite::SqlitePool;
use tokens::TokensRepository;
use users::UsersRepository;

#[derive(Clone, Debug)]
pub struct Database {
    pub quotes: QuotesRepository,
    pub tokens: TokensRepository,
    pub users: UsersRepository,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let pool = SqlitePool::connect(database_url).await.unwrap();
        Database {
            quotes: QuotesRepository::new(pool.clone()),
            tokens: TokensRepository::new(pool.clone()),
            users: UsersRepository::new(pool.clone()),
        }
    }
}
