use anyhow::Result;
use sqlx::PgPool;

use crate::config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: PgPool,
}
impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = PgPool::connect(config::DATABASE_URL).await?;

        Ok(Self { database: pool })
    }
}
