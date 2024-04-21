use chrono::NaiveDateTime;

use crate::enums::ProgrammingLanguage;

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct SubmitHistory {
    pub id: String,
    pub score: f32,
    pub language: ProgrammingLanguage,
    pub submissions: String,
    pub submitted_at: NaiveDateTime,
    pub time: Option<i32>,
    pub space: Option<i32>,
    pub link: Option<String>,
    pub account_id: Option<String>,
    pub question_id: Option<String>
}
