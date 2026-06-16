use crate::models::Invite;
use sqlx::sqlite::SqlitePool;

#[derive(Clone, Debug)]
pub struct InvitesRepository {
    pool: SqlitePool,
}

impl InvitesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        InvitesRepository { pool }
    }

    pub async fn get(&self, id: &str) -> Result<Option<Invite>, sqlx::Error> {
        sqlx::query_as!(Invite, "SELECT * FROM invites WHERE id = ?1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn insert(&self, token: &Invite) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO invites VALUES (?1, ?2)",
            token.id,
            token.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM invites WHERE id = ?1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
