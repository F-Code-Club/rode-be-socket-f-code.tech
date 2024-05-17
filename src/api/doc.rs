use utoipa::OpenApi;

use super::controller;
use crate::{enums, util};

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::ping,
        controller::scoring::run,
        controller::scoring::submit,
        controller::scoring::render_diff_image,
        controller::auth::login,
        controller::auth::refresh,
        controller::auth::session_socket
    ),
    components(schemas(
        controller::scoring::SubmitData,
        controller::scoring::RenderDiffImageData,
        controller::auth::LoginData,
        controller::auth::TokenPair,
        enums::ProgrammingLanguage,
        util::scoring::ExecutionResult,
        crate::error::ErrorResponse,
    ))
)]
pub struct ApiDoc;
