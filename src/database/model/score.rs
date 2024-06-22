use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct Score {
    pub id: Uuid,
    pub room_id: i32,
    pub team_id: i32,
    pub total_score: i32,
    pub last_submit_time: NaiveDateTime,
    pub penalty: i32,
}

impl Score {
    #[tracing::instrument(err)]
    pub async fn get(room_id: i32, team_id: i32, database: &PgPool) -> anyhow::Result<Score> {
        let score = sqlx::query_as!(
            Score,
            "SELECT * FROM scores WHERE room_id = $1 AND team_id = $2",
            room_id,
            team_id
        )
        .fetch_one(database)
        .await?;

        Ok(score)
    }
}
