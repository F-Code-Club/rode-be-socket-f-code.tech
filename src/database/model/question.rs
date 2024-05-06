use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Question {
    pub id: uuid::fmt::Hyphenated,
    pub stack_id: uuid::fmt::Hyphenated,
    // Always >= 1
    pub max_submit_time: i32,
    pub score: u32
}
