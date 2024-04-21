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

#[derive(Debug, sqlx::Type)]
pub enum ProgrammingLanguage {
    #[sqlx(rename = "C_CPP")]
    C_CPP,
    #[sqlx(rename = "PYTHON")]
    Python,
    #[sqlx(rename = "JAVA")]
    Java
}
impl From<String> for ProgrammingLanguage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "C_CPP" => Self::C_CPP,
            "PYTHON" => Self::Python,
            "Java" => Self::Java,
            _ => unreachable!()
        }
    }
}

