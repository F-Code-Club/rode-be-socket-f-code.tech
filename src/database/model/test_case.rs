use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, sqlx::FromRow)]
pub struct TestCase {
    pub id: i32,
    pub question_id: Uuid,
    pub input: String,
    pub is_visible: bool,
    pub output: String,
}

impl TestCase {
    pub async fn get_many_by_question_id_internal(
        question_id: Uuid,
        database: &PgPool,
    ) -> sqlx::Result<Vec<TestCase>> {
        sqlx::query_as!(
            TestCase,
            "SELECT * FROM test_cases WHERE question_id = $1",
            question_id
        )
        .fetch_all(database)
        .await
    }

    #[tracing::instrument(err)]
    pub async fn get_many_by_question_id(
        question_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Vec<TestCase>> {
        // TODO: find best cache size
        const CACHE_SIZE: u64 = 50;
        static CACHE: OnceCell<Cache<Uuid, Vec<TestCase>>> = OnceCell::const_new();

        let cache = CACHE.get_or_init(|| async { Cache::new(CACHE_SIZE) }).await;

        match cache
            .try_get_with(
                question_id,
                TestCase::get_many_by_question_id_internal(question_id, database),
            )
            .await
        {
            Ok(test_cases) => Ok(test_cases),
            Err(error) => anyhow::bail!(error.to_string()),
        }
    }
}
