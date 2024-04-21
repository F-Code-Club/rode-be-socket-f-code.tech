#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Question {
    pub id: String,
    #[sqlx(rename = "questionImage")]
    pub image: String,
    pub max_submit_times: i32,
    pub colors: String,
    pub code_template: Option<String>,
    pub room_id: String
}
