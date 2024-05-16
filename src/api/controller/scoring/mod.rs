use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::enums::ProgrammingLanguage;

mod render_diff;
mod run;
mod submit;

pub use render_diff::*;
pub use run::*;
pub use submit::*;

#[derive(Deserialize, ToSchema)]
pub struct Data {
    room_id: i32,
    question_id: Uuid,
    language: ProgrammingLanguage,
    code: String,
}
