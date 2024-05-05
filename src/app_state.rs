use anyhow::Result;
use sqlx::MySqlPool;

use crate::config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: MySqlPool,
}
impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = MySqlPool::connect(config::DATABASE_URL).await?;

        Ok(Self { database: pool })
    }
}
