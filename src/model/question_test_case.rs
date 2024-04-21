#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct QuestionTestCase {
    pub id: i32,
    pub input: String,
    pub output: String,
    pub question_id: String,
}
