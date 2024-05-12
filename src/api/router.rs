use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::controller;
use super::doc::ApiDoc;
use crate::app_state::AppState;

pub fn build(state: Arc<AppState>) -> Router {
    // register routes
    let router = Router::new()
        .route("/", get(controller::ping))
        .route("/scoring/run", post(controller::scoring::run))
        .route("/scoring/submit", post(controller::scoring::submit))
        .route("/team/join", post(controller::team::join));

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    router.with_state(state)
}
