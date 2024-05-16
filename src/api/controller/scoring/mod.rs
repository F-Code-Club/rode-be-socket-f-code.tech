use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::enums::ProgrammingLanguage;

mod run;
mod submit;

pub use run::*;
pub use submit::*;

#[derive(Deserialize, ToSchema)]
pub struct Data {
    room_id: i32,
    question_id: Uuid,
    language: ProgrammingLanguage,
    code: String,
}
