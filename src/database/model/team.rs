use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Team {
    pub id: i32,
    pub name: String,
    // Always >= 1
    pub member_count: i32,
}
