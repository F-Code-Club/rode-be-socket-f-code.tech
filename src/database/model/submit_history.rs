use chrono::NaiveDateTime;
use serde::Serialize;

use crate::enums::ProgrammingLanguage;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubmitHistory {
    pub question_id: uuid::fmt::Hyphenated,
    pub score_id: u32,
    pub member_id: uuid::fmt::Hyphenated,
    // Always >= 1
    pub submit_number: i32,
    pub run_time: u32,
    pub score: u32,
    pub language: ProgrammingLanguage,
    pub character_count: u32,
    pub last_submit_time: NaiveDateTime,
    pub submissions: String,
}
