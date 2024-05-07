use chrono::NaiveDate;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::enums::AccountRole;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub full_name: String,
    pub student_id: String,
    pub email: String,
    pub password: String,
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
    pub async fn get_one_by_id(id: Uuid, database: &PgPool) -> anyhow::Result<Account> {
        let account =
            sqlx::query_as_unchecked!(Account, "SELECT * FROM accounts WHERE id = $1", id)
                .fetch_one(database)
                .await?;

        Ok(account)
    }
}
