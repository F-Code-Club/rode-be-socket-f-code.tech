mod update_score;

use std::time::Duration;

use sqlx::PgPool;
use tokio::time::sleep;

use crate::config;

pub async fn start() {
    let database = PgPool::connect(&config::DATABASE_URL).await.unwrap();
    loop {
        sleep(Duration::from_mins(*config::DATABASE_CRON_JOB_INTERVAL)).await;
        let _ = update_score::update_score(&database).await;
    }
}
