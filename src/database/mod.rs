mod invites;
mod login_codes;
mod quotes;
mod users;

use invites::InvitesRepository;
use login_codes::LoginCodesRepository;
use quotes::QuotesRepository;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;
use users::UsersRepository;

#[derive(Clone, Debug)]
pub struct Database {
    pub quotes: QuotesRepository,
    pub invites: InvitesRepository,
    pub users: UsersRepository,
    pub login_codes: LoginCodesRepository,
}

impl Database {
    pub async fn new(database_url: &str) -> Self {
        let options = SqliteConnectOptions::from_str(database_url)
            .unwrap()
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await.unwrap();
        Database {
            quotes: QuotesRepository::new(pool.clone()),
            invites: InvitesRepository::new(pool.clone()),
            users: UsersRepository::new(pool.clone()),
            login_codes: LoginCodesRepository::new(pool.clone()),
        }
    }
}
