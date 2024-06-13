use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema, sqlx::FromRow)]
pub struct Question {
    /// Id of the question
    ///
    /// # Notes
    /// - Skipped during serializing since id is only relevant to server side
    #[serde(skip)]
    pub id: Uuid,

    /// Id of the stack containing the question
    ///
    /// # Notes
    /// - Skipped during serializing since stack id is only relevant to server side
    #[serde(skip)]
    pub stack_id: Uuid,

    /// Max submit time of the question
    ///
    /// # Constraints
    /// - max_submit_time >= 1
    pub max_submit_time: i32,

    /// Score a team will get if finished the question
    ///
    /// # Constraints
    /// - score >= 0
    pub score: i32,
}

impl Question {
    async fn get_one_by_ids_internal(
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

    #[tracing::instrument(err)]
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
