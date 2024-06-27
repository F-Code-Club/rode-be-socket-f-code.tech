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
    pub async fn get(room_code: String, team_id: i32, database: &PgPool) -> anyhow::Result<Score> {
        let score = sqlx::query_as!(
            Score,
            r#"
            SELECT scores.*
            FROM scores 
            INNER JOIN rooms ON rooms.id = scores.room_id
            WHERE rooms.code = $1 AND team_id = $2
            "#,
            room_code,
            team_id
        )
        .fetch_one(database)
        .await?;

        Ok(score)
    }

    pub async fn update(
        score: Score,
        additional_penalty: i32,
        now: NaiveDateTime,
        database: &PgPool,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                UPDATE scores
                SET last_submit_time = $2, penalty = $3
                WHERE id = $1
            "#,
            score.id,
            now,
            score.penalty + additional_penalty
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
