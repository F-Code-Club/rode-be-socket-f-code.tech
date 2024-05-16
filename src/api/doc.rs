use utoipa::OpenApi;

use super::controller;
use crate::Error;
use crate::{enums, util};

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::ping,
        controller::scoring::run,
        controller::scoring::submit,
        controller::scoring::render_diff,
        controller::room::join,
    ),
    components(schemas(
        controller::scoring::Data,
        controller::scoring::RenderDiffParam,
        controller::room::JoinRoomInfo,
        enums::ProgrammingLanguage,
        util::scoring::ExecutionResult,
        Error,
    ))
)]
pub struct ApiDoc;
