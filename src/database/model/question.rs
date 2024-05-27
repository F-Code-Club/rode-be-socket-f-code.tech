use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Question {
    pub id: Uuid,
    pub stack_id: Uuid,
    // Always >= 1
    pub max_submit_time: i32,
    pub score: i32,
}

impl Question {
    pub async fn get_one_by_ids_internal(
        id: Uuid,
        stack_id: Uuid,
        database: &PgPool,
    ) -> sqlx::Result<Question> {
        sqlx::query_as!(
            Question,
            "SELECT * FROM questions WHERE id = $1 AND stack_id = $2",
            id,
            stack_id
        )
        .fetch_one(database)
        .await
    }

    pub async fn get_one_by_ids(
        id: Uuid,
        stack_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Question> {
        // TODO: find best cache size
        const CACHE_SIZE: u64 = 50;
        static CACHE: OnceCell<Cache<(Uuid, Uuid), Question>> = OnceCell::const_new();

        let cache = CACHE.get_or_init(|| async { Cache::new(CACHE_SIZE) }).await;

        match cache
            .try_get_with(
                (id, stack_id),
                Question::get_one_by_ids_internal(id, stack_id, database),
            )
            .await
        {
            Ok(question) => Ok(question),
            Err(error) => anyhow::bail!(error.to_string()),
        }
    }
}
