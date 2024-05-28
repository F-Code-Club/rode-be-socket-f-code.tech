use chrono::NaiveDate;
use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::enums::AccountRole;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub full_name: String,
    pub student_id: String,
    pub email: String,
    /// password is not None when is_enabled == true
    pub password: Option<String>,
    pub phone: String,
    pub dob: NaiveDate,
    pub role: AccountRole,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub is_locked: bool,
    pub is_logged_in: bool,
    pub is_enabled: bool,
}

impl Account {
    async fn get_one_by_id_id_internal(id: Uuid, database: &PgPool) -> sqlx::Result<Account> {
        sqlx::query_as_unchecked!(Account, "SELECT * FROM accounts WHERE id = $1", id)
            .fetch_one(database)
            .await
    }

    pub async fn get_one_by_id(id: Uuid, database: &PgPool) -> anyhow::Result<Account> {
        // TODO: find best cache size
        const CACHE_SIZE: u64 = 50;
        static CACHE: OnceCell<Cache<Uuid, Account>> = OnceCell::const_new();

        let cache = CACHE.get_or_init(|| async { Cache::new(CACHE_SIZE) }).await;

        match cache
            .try_get_with(id, Account::get_one_by_id_id_internal(id, database))
            .await
        {
            Ok(account) => Ok(account),
            Err(error) => anyhow::bail!(error.to_string()),
        }
    }

    pub async fn get_one_by_email(email: &str, database: &PgPool) -> anyhow::Result<Account> {
        let account =
            sqlx::query_as_unchecked!(Account, "SELECT * FROM accounts WHERE email = $1", email)
                .fetch_one(database)
                .await?;

        Ok(account)
    }
}
