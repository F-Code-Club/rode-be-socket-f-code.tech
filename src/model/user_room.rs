use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct UserRoom {
    pub id: String,
    pub join_time: NaiveDateTime,
    pub finish_time: Option<NaiveDateTime>,
    pub attendance: bool,
    pub account_id: String,
    pub room_id: String
}
