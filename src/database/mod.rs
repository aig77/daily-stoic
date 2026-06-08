mod invites;
mod login_codes;
mod quotes;
mod users;

use invites::InvitesRepository;
use login_codes::LoginCodesRepository;
use quotes::QuotesRepository;
use sqlx::sqlite::SqlitePool;
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
        let pool = SqlitePool::connect(database_url).await.unwrap();
        Database {
            quotes: QuotesRepository::new(pool.clone()),
            invites: InvitesRepository::new(pool.clone()),
            users: UsersRepository::new(pool.clone()),
            login_codes: LoginCodesRepository::new(pool.clone()),
        }
    }
}
