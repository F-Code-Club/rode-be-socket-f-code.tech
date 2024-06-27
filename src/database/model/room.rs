use chrono::{NaiveDate, NaiveDateTime};
use moka::future::Cache;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::enums::RoomKind;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
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
    pub async fn get_one_by_code(code: &str, database: &PgPool) -> anyhow::Result<Room> {
        let room = sqlx::query_as_unchecked!(Room, "SELECT * FROM rooms WHERE code = $1", code)
            .fetch_one(database)
            .await?;

        Ok(room)
    }

    async fn get_one_by_id_internal(id: i32, database: &PgPool) -> sqlx::Result<Room> {
        sqlx::query_as_unchecked!(Room, "SELECT * FROM rooms WHERE id = $1", id)
            .fetch_one(database)
            .await
    }

    #[tracing::instrument(err)]
    pub async fn get_one_by_id(id: i32, database: &PgPool) -> anyhow::Result<Room> {
        // TODO: find best cache size
        const CACHE_SIZE: u64 = 50;
        static CACHE: OnceCell<Cache<i32, Room>> = OnceCell::const_new();

        let cache = CACHE.get_or_init(|| async { Cache::new(CACHE_SIZE) }).await;

        match cache
            .try_get_with(id, Room::get_one_by_id_internal(id, database))
            .await
        {
            Ok(room) => Ok(room),
            Err(error) => anyhow::bail!(error.to_string()),
        }
    }

    pub fn is_open(&self, time: NaiveDateTime) -> bool {
        self.open_time < time && time < self.close_time
    }
}
