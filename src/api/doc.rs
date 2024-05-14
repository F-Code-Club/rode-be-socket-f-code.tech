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
        controller::room::join,
    ),
    security(("api_key" = [])),
    components(schemas(
        controller::scoring::Data,
        controller::room::JoinRoomInfo,
        enums::ProgrammingLanguage,
        util::scoring::ExecutionResult,
        Error,
    ))
)]
pub struct ApiDoc;
