use serde::Serialize;

#[derive(Debug, Default, Serialize, sqlx::FromRow)]
pub struct Testcase {
    pub id: i32,
    pub question_id: uuid::fmt::Hyphenated,
    pub input: String,
    pub output: String,
}
