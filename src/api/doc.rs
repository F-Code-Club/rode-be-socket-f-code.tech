use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use super::controller;
use crate::{enums, util};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::ping,
        controller::scoring::run,
        controller::scoring::submit,
        controller::scoring::render_diff_image,
        controller::room::join,
        controller::team::get_id,
        controller::editor_socket,
        controller::auth::login,
        controller::auth::logout,
        controller::auth::refresh,
        controller::auth::session_socket
    ),
    modifiers(&SecurityAddon),
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
