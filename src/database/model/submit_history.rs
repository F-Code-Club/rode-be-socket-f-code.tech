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
    #[tracing::instrument(err)]
    pub async fn insert(self, database: &PgPool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO submit_histories (question_id, score_id, member_id, submit_number, run_time, score, language, character_count, last_submit_time, submissions)
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            self.question_id,
            self.score_id,
            self.member_id,
            self.submit_number,
            self.run_time,
            self.score,
            self.language as _,
            self.character_count,
            self.last_submit_time,
            self.submissions
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
