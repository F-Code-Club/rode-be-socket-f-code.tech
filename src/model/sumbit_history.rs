use chrono::NaiveDateTime;

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

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct SubmitHistory {
    pub id: String,
    pub score: f32,
    pub language: ProgrammingLanguage,
    pub submissions: String,
    pub submitted_at: NaiveDateTime,
    pub time: Option<i32>,
    pub space: Option<i32>,
    pub link: Option<String>,
    pub account_id: Option<String>,
    pub question_id: Option<String>
}
