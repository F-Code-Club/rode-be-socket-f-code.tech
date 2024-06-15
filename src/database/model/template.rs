use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema, sqlx::FromRow)]
pub struct Template {
    /// Id of the template
    ///
    /// # Notes
    /// - Skipped during serializing since id is only relevant to server side
    #[serde(skip)]
    pub id: Uuid,

    /// Id of the question containing the template
    ///
    /// # Notes
    /// - Skipped during serializing since question id is only relevant to server side
    #[serde(skip)]
    pub question_id: Uuid,

    /// Path of the template files, relative to the root folder specified in [url](Template::url)
    pub local_path: String,

    /// Google drive link to the root folder
    #[serde(skip)]
    pub url: String,
}

impl Template {
    async fn get_one_by_question_id_internal(
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

    #[tracing::instrument(err)]
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
