use chrono::NaiveDateTime;

#[derive(Debug, sqlx::Type)]
pub enum AccountRole {
    #[sqlx(rename = "user")]
    User,
    #[sqlx(rename = "admin")]
    Admin,
}
impl From<String> for AccountRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "user" => AccountRole::User,
            "admin" => AccountRole::Admin,
            _ => unreachable!(),
        }
    }
}

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
