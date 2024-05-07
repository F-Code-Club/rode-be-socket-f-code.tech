use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct Testcase {
    pub id: i32,
    pub question_id: Uuid,
    pub input: String,
    pub output: String,
}
