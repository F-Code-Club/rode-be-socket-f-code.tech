use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::enums::ProgrammingLanguage;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubmitHistory {
    pub question_id: Uuid,
    pub score_id: Uuid,
    pub member_id: i32,
    // Always >= 1
    pub submit_number: i32,
    pub run_time: i32,
    pub score: i32,
    pub language: ProgrammingLanguage,
    pub character_count: i32,
    pub last_submit_time: NaiveDateTime,
    pub submissions: String,
}

impl SubmitHistory {
    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(err)]
    pub async fn insert(
        question_id: Uuid,
        score_id: Uuid,
        member_id: i32,
        submit_number: i32,
        run_time: i32,
        score: i32,
        language: ProgrammingLanguage,
        character_count: i32,
        last_submit_time: NaiveDateTime,
        submissions: String,
        database: &PgPool,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO submit_histories (question_id, score_id, member_id, submit_number, run_time, score, language, character_count, last_submit_time, submissions)
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            question_id,
            score_id,
            member_id,
            submit_number,
            run_time,
            score,
            language as _,
            character_count,
            last_submit_time,
            submissions
        )
        .execute(database)
        .await?;

        Ok(())
    }

    /// Count the number of submit for a question of a team
    #[tracing::instrument(err)]
    pub async fn count(question_id: Uuid, team_id: i32, database: &PgPool) -> anyhow::Result<i32> {
        let submit_count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(score_id)
            FROM submit_histories
            INNER JOIN members ON submit_histories.member_id = members.id
            WHERE question_id = $1 AND team_id = $2
            "#,
            question_id,
            team_id
        )
        .fetch_one(database)
        .await?
        .unwrap_or(0) as i32;

        Ok(submit_count)
    }
}
