use chrono::NaiveDate;
use serde::Serialize;

use crate::enums::{QuestionStackStatus, RoomKind};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QuestionStack {
    pub id: uuid::fmt::Hyphenated,
    // Always >= 1
    pub stack_max: i32,
    pub name: String,
    pub status: QuestionStackStatus,
    pub created_at: NaiveDate,
    #[sqlx(rename = "type")]
    pub kind: RoomKind,
}
