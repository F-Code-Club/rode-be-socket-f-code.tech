use chrono::NaiveDateTime;
use crate::enums::RoomKind;

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Room {
    pub id: String,
    pub code: String,
    pub open_time: NaiveDateTime,
    pub close_time: Option<NaiveDateTime>,
    pub duration: Option<i32>,
    #[sqlx(rename = "type")]
    pub kind: RoomKind,
    pub is_private: bool,
    pub created_at: NaiveDateTime
}
