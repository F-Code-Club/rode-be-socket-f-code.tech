use anyhow::Result;
use sqlx::MySqlPool;
use dotenvy_macro::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub database: MySqlPool,
}
impl AppState {
    pub async fn new() -> Result<Self> {
        let pool = MySqlPool::connect(dotenv!("DATABASE_URL")).await?;

        // create tables if not existed
        //pool.execute(include_str!("./schema.sql")).await?;

        Ok(Self { database: pool })
    }
}
