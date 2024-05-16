use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use chromiumoxide::Method;
use google_drive3::hyper::header::HeaderValue;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::controller;
use super::doc::ApiDoc;
use crate::{app_state::AppState, config};

pub fn build(state: Arc<AppState>) -> Router {
    // register routes
    let router = Router::new()
        .route("/", get(controller::ping))
        .route("/auth/login", post(controller::auth::login))
        .route("/auth/refresh", post(controller::auth::refresh))
        .route(
            "/auth/session/socket",
            get(controller::auth::session_socket),
        )
        .route("/scoring/run", post(controller::scoring::run))
        .route("/scoring/submit", post(controller::scoring::submit))
        .layer(
            CorsLayer::new()
                .allow_origin(config::PUBLIC_CORS_DOMAIN.parse::<HeaderValue>().unwrap())
                .allow_origin(config::LOCAL_CORS_DOMAIN.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        );

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    router.with_state(state)
}
