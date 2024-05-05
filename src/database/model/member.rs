use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    pub team_id: u32,
    pub account_id: uuid::fmt::Hyphenated,
    pub has_joined_room: bool,
}
