use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::enums::{QuestionStackStatus, RoomKind};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct QuestionStack {
    pub id: Uuid,
    // Always >= 1
    pub stack_max: i32,
    pub name: String,
    pub status: QuestionStackStatus,
    pub created_at: NaiveDate,
    #[sqlx(rename = "type")]
    pub kind: RoomKind,
}
