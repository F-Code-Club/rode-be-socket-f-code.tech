use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::enums::ProgrammingLanguage;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubmitHistory {
    pub question_id: Uuid,
    pub score_id: Uuid,
    pub member_id: i32,
    // Always >= 1
    pub submit_number: i32,
    #[sqlx(try_from = "i32")]
    pub run_time: u32,
    #[sqlx(try_from = "i32")]
    pub score: u32,
    pub language: ProgrammingLanguage,
    #[sqlx(try_from = "i32")]
    pub character_count: u32,
    pub last_submit_time: NaiveDateTime,
    pub submissions: String,
}
