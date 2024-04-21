use chrono::NaiveDateTime;

#[derive(Debug, sqlx::Type)]
pub enum RoomKind {
    #[sqlx(rename = "BE")]
    Backend,
    #[sqlx(rename = "FE")]
    Frontend
}
impl From<String> for RoomKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BE" => Self::Backend,
            "FE" => Self::Frontend,
            _ => unreachable!()
        }
    }
}

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
