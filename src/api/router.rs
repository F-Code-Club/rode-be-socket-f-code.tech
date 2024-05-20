use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::controller;
use super::doc::ApiDoc;
use crate::{app_state::AppState, config, util::timeout_handler::handle_timeout_error};

pub fn build(state: Arc<AppState>) -> Router {
    let allow_origins = vec![
        config::PUBLIC_CORS_DOMAIN.parse::<HeaderValue>().unwrap(),
        config::LOCAL_CORS_DOMAIN.parse::<HeaderValue>().unwrap(),
    ];

    let middleware = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .timeout(Duration::from_secs(5));

    // register routes
    let router = Router::new()
        .route("/", get(controller::ping))
        .route("/auth/login", post(controller::auth::login))
        .route("/auth/refresh", post(controller::auth::refresh))
        .route(
            "/auth/session/socket",
            get(controller::auth::session_socket),
        )
        .route("/room/join", post(controller::room::join))
        .route("/team/get-id", get(controller::team::get_id))
        .route(
            "/editor/socket/:question_id/:team_id",
            get(controller::editor_socket),
        )
        .route(
            "/scoring/run",
            post(controller::scoring::run).layer(middleware.clone()),
        )
        .route(
            "/scoring/submit",
            post(controller::scoring::submit).layer(middleware.clone()),
        )
        .route(
            "/scoring/render-diff-image",
            post(controller::scoring::render_diff_image).layer(middleware.clone()),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origins)
                .allow_methods([Method::GET, Method::POST]),
        );

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    router.with_state(state)
}
