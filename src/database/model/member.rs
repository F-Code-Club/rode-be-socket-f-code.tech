use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    #[sqlx(try_from = "i32")]
    pub team_id: u32,
    pub account_id: Uuid,
    pub has_joined_room: bool,
}
