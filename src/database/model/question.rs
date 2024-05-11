use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Question {
    pub id: Uuid,
    pub stack_id: Uuid,
    // Always >= 1
    pub max_submit_time: i32,
    pub score: i32,
}

impl Question {
    pub async fn get_one_by_ids(
        id: Uuid,
        stack_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Question> {
        let question = sqlx::query_as!(
            Question,
            "SELECT * FROM questions WHERE id = $1 AND stack_id = $2",
            id,
            stack_id
        )
        .fetch_one(database)
        .await?;

        Ok(question)
    }
}
