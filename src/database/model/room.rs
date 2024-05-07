use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use uuid::Uuid;

use crate::enums::RoomKind;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Room {
    pub id: i32,
    pub code: String,
    pub stack_id: Uuid,
    // Always >= 1
    pub size: i32,
    #[sqlx(rename = "type")]
    pub kind: RoomKind,
    pub open_time: NaiveDateTime,
    pub close_time: NaiveDateTime,
    pub created_at: NaiveDate,
    pub is_privated: bool,
}
