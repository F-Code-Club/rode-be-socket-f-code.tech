use std::collections::HashSet;
use std::sync::Mutex;

use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config;

#[derive(Debug)]
pub struct AppState {
    pub database: PgPool,
    pub logged_in_account_ids: Mutex<HashSet<Uuid>>,
}
impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = PgPool::connect(&config::DATABASE_URL).await?;

        Ok(Self {
            database: pool,
            logged_in_account_ids: Mutex::new(HashSet::new()),
        })
    }
}
