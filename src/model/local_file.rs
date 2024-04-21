#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct LocalFile {
    pub id: String,
    pub path: String,
    pub is_used: bool
}
