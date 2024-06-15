use anyhow::Result;
use dashmap::DashMap;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config;

#[derive(Debug)]
pub struct AppState {
    pub database: PgPool,
    pub account_fingerprints: DashMap<Uuid, String>
}
impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = PgPool::connect(&config::DATABASE_URL).await?;

        Ok(Self {
            database: pool,
            account_fingerprints: DashMap::new()
        })
    }
}
