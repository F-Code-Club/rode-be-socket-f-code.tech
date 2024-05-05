use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Template {
    pub id: i32,
    pub question_id: uuid::fmt::Hyphenated,
    pub local_path: String,
    pub url: String
}
