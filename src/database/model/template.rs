use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Template {
    pub id: i32,
    pub question_id: Uuid,
    pub local_path: String,
    pub url: String
}
