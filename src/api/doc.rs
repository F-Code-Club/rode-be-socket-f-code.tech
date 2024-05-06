use utoipa::OpenApi;

use super::controller;

#[derive(OpenApi)]
#[openapi(paths(controller::ping))]
pub struct ApiDoc;
