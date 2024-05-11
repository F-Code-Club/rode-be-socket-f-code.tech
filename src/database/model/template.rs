use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Template {
    pub id: Uuid,
    pub question_id: Uuid,
    pub local_path: String,
    pub url: String,
}

impl Template {
    pub async fn get_one_by_question_id(
        question_id: Uuid,
        database: &PgPool,
    ) -> anyhow::Result<Template> {
        let template = sqlx::query_as!(
            Template,
            "SELECT * FROM templates WHERE question_id = $1",
            question_id
        )
        .fetch_one(database)
        .await?;

        Ok(template)
    }
}
