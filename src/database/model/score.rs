use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Score {
    pub id: Uuid,
    pub room_id: i32,
    pub team_id: Uuid,
    #[sqlx(try_from = "i32")]
    pub total_scores: u32,
    pub last_submit_time: NaiveDateTime,
    #[sqlx(try_from = "i32")]
    pub penalty: u32,
}
