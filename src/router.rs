use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::controller;
use crate::util::app_state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::ping
    ),
)]
struct ApiDoc;

pub fn build(state: AppState) -> Router {
    // register routes
    let router = Router::new().route("/", get(controller::ping));

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    router.with_state(state)
}
