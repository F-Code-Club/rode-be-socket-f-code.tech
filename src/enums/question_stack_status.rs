use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuestionStackStatus {
    Draft,
    Active,
    DeActive,
    Used
}
impl From<String> for QuestionStackStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DRAFT" => QuestionStackStatus::Draft,
            "ACTIVE" => QuestionStackStatus::Active,
            "DE_ACTIVE" => QuestionStackStatus::DeActive,
            "USED" => QuestionStackStatus::Used,
            _ => unreachable!()
        }
    }
}
