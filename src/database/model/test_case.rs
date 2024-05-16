use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct TestCase {
    pub id: i32,
    pub question_id: Uuid,
    pub input: String,
    pub output: String,
}

impl TestCase {
    pub async fn get_many_by_question_id(
        question_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Vec<TestCase>> {
        let test_cases = sqlx::query_as!(
            TestCase,
            "SELECT * FROM test_cases WHERE question_id = $1",
            question_id
        )
        .fetch_all(database)
        .await?;

        Ok(test_cases)
    }
}
