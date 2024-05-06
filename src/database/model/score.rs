use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Score {
    pub id: uuid::fmt::Hyphenated,
    pub room_id: i32,
    pub team_id: uuid::fmt::Hyphenated,
    pub total_scores: u32,
    pub last_submit_time: NaiveDateTime,
}
