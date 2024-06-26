use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, sqlx::Type)]
#[sqlx(type_name = "type_enum")]
pub enum RoomKind {
    #[sqlx(rename = "BE")]
    #[serde(rename = "BE")]
    Backend,
    #[sqlx(rename = "FE")]
    #[serde(rename = "FE")]
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
