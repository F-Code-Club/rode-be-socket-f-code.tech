use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::enums::RoomKind;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Room {
    pub id: i32,
    pub code: String,
    pub stack_id: Uuid,
    // Always >= 1
    pub size: i32,
    pub r#type: RoomKind,
    pub open_time: NaiveDateTime,
    pub close_time: NaiveDateTime,
    pub created_at: NaiveDate,
    pub is_privated: bool,
}

impl Room {
    pub async fn get_one_by_id(id: i32, database: &PgPool) -> anyhow::Result<Room> {
        let room = sqlx::query_as_unchecked!(Room, "SELECT * FROM rooms WHERE id = $1", id)
            .fetch_one(database)
            .await?;

        Ok(room)
    }
    
    pub fn is_open(&self, time: NaiveDateTime) -> bool {
        self.open_time < time && time < self.close_time 
    }
}
