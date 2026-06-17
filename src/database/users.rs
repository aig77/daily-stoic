use crate::models::User;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct UsersRepository {
    pool: SqlitePool,
}

impl UsersRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?1", email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn insert(&self, email: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO users (email) VALUES (?1)", email,)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_email_settings(
        &self,
        email: &str,
        emails_enabled: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET emails_enabled = ?1 WHERE email = ?2",
            emails_enabled,
            email,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_schedule(
        &self,
        email: &str,
        send_time: &str,
        timezone: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET send_time = ?1, timezone = ?2 WHERE email = ?3",
            send_time,
            timezone,
            email,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, email: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE email = ?1", email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_all_enabled(&self) -> Result<Vec<User>, sqlx::Error> {
        Ok(
            sqlx::query_as!(User, "SELECT * FROM users WHERE emails_enabled = 1")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn grant_admin(&self, email: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE users SET is_admin = 1 WHERE email = ?1", email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_scheduled_users(&self, send_time: &str) -> Result<Vec<String>, sqlx::Error> {
        Ok(sqlx::query_scalar!(
            "SELECT email FROM users WHERE emails_enabled = 1 AND send_time = ?1",
            send_time
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_admins(&self) -> Result<Vec<String>, sqlx::Error> {
        Ok(
            sqlx::query_scalar!("SELECT email FROM users WHERE is_admin = 1")
                .fetch_all(&self.pool)
                .await?,
        )
    }
}
