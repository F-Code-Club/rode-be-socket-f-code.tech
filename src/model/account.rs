use chrono::NaiveDateTime;
use crate::enums::AccountRole;

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub student_id: String,
    pub phone: String,
    pub dob: NaiveDateTime,
    pub role: AccountRole,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub is_logged_in: bool,
}
