use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::enums::ProgrammingLanguage;

mod render_diff_image;
mod run;
mod submit;

pub use render_diff_image::*;
pub use run::*;
pub use submit::*;

#[derive(Deserialize, ToSchema)]
pub struct SubmitData {
    room_id: i32,
    #[schema(value_type = String, format = Uuid)]
    question_id: Uuid,
    language: ProgrammingLanguage,
    code: String,
}
