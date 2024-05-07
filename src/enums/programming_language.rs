use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "programming_lang_enum", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgrammingLanguage {
    #[allow(non_camel_case_types)]
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
            ProgrammingLanguage::C_CPP => "c",
            ProgrammingLanguage::Python => "py",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::Css => "css",
        }
    }
}
