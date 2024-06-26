use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "programming_lang_enum", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgrammingLanguage {
    #[allow(non_camel_case_types)]
    #[sqlx(rename = "C_CPP")]
    #[serde(rename = "C_CPP")]
    C_CPP,
    Python,
    Java,
    Css
}
impl From<String> for ProgrammingLanguage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "C_CPP" => Self::C_CPP,
            "PYTHON" => Self::Python,
            "JAVA" => Self::Java,
            "CSS" => Self::Css,
            _ => unreachable!()
        }
    }
}
impl ProgrammingLanguage {
    pub fn get_extension(&self) -> &'static str {
        match self {
            ProgrammingLanguage::C_CPP => "cpp",
            ProgrammingLanguage::Python => "py",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::Css => "css",
        }
    }
}
