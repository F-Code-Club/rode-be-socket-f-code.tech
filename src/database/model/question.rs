use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Question {
    pub id: Uuid,
    pub stack_id: Uuid,
    // Always >= 1
    pub max_submit_time: i32,
    #[sqlx(try_from = "i32")]
    pub score: u32
}
