use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Template {
    pub id: Uuid,
    pub question_id: Uuid,
    pub local_path: String,
    pub url: String,
}

impl Template {
    pub async fn get_one_by_question_id_internal(
        question_id: Uuid,
        database: &PgPool,
    ) -> sqlx::Result<Template> {
        sqlx::query_as!(
            Template,
            "SELECT * FROM templates WHERE question_id = $1",
            question_id
        )
        .fetch_one(database)
        .await
    }

    pub async fn get_one_by_question_id(
        question_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Template> {
        // TODO: find best cache size
        const CACHE_SIZE: u64 = 50;
        static CACHE: OnceCell<Cache<Uuid, Template>> = OnceCell::const_new();

        let cache = CACHE.get_or_init(|| async { Cache::new(CACHE_SIZE) }).await;

        match cache
            .try_get_with(
                question_id,
                Template::get_one_by_question_id_internal(question_id, database),
            )
            .await
        {
            Ok(template) => Ok(template),
            Err(error) => anyhow::bail!(error.to_string()),
        }
    }
}
