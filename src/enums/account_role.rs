use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, sqlx::Type)]
#[sqlx(type_name = "role_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AccountRole {
    Admin,
    Manager,
    User,
}
impl From<String> for AccountRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "admin" => AccountRole::Admin,
            "manager" => AccountRole::Manager,
            "user" => AccountRole::User,
            _ => unreachable!(),
        }
    }
}
