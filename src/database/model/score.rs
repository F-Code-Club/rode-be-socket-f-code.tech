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
    pub async fn get_optional_by_ids(
        room_id: i32,
        team_id: i32,
        database: &PgPool,
    ) -> anyhow::Result<Option<Score>> {
        let score = sqlx::query_as!(
            Score,
            "SELECT * FROM scores WHERE room_id = $1 AND team_id = $2",
            room_id,
            team_id
        )
        .fetch_optional(database)
        .await?;

        Ok(score)
    }

    pub async fn insert(self, database: &PgPool) -> anyhow::Result<Uuid> {
        let id = sqlx::query_scalar!(
            r#"
                INSERT INTO scores (room_id, team_id, total_score, last_submit_time, penalty)
                VALUES($1, $2, $3, $4, $5)
                RETURNING id
                "#,
            self.room_id,
            self.team_id,
            self.total_score,
            self.last_submit_time,
            self.penalty,
        )
        .fetch_one(database)
        .await?;

        Ok(id)
    }

    /// Create a score record for a team in a room if not existed,
    /// then update the penalty and last submit time
    pub async fn update_or_insert(
        room_id: i32,
        team_id: i32,
        additional_penalty: i32,
        now: NaiveDateTime,
        database: &PgPool,
    ) -> anyhow::Result<Uuid> {
        let score_id = match Score::get_optional_by_ids(room_id, team_id, database).await? {
            None => {
                let new_score = Score {
                    room_id,
                    team_id,
                    total_score: 0,
                    last_submit_time: now,
                    penalty: additional_penalty,
                    ..Default::default()
                };
                new_score.insert(database).await?
            }
            Some(score) => {
                let (id, old_penalty) = (score.id, score.penalty);
                sqlx::query!(
                    r#"
                UPDATE scores
                SET last_submit_time = $2, penalty = $3
                WHERE id = $1
                "#,
                    id,
                    now,
                    old_penalty + additional_penalty
                )
                .execute(database)
                .await?;

                id
            }
        };

        Ok(score_id)
    }
}
