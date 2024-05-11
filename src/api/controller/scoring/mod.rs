use serde::Deserialize;
use uuid::Uuid;

use crate::enums::ProgrammingLanguage;

mod run;
mod submit;

pub use submit::*;
pub use run::*;

#[derive(Deserialize)]
pub struct Data {
    room_id: i32,
    question_id: Uuid,
    language: ProgrammingLanguage,
    code: String,
}
