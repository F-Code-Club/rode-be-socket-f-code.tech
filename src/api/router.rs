use axum::extract::{MatchedPath, Request};
use axum::http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Router,
};
use std::time::Instant;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::controller;
use super::doc::ApiDoc;
use crate::{app_state::AppState, config, util::timeout_handler::handle_timeout_error};

const ALLOW_HEADERS: [HeaderName; 7] = [
    ORIGIN,
    AUTHORIZATION,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
    ACCEPT,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_HEADERS,
];
const ALLOW_METHODS: [Method; 2] = [Method::GET, Method::POST];

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    response
}

pub fn build(state: Arc<AppState>) -> Router {
    let allow_origins = [
        config::PUBLIC_CORS_DOMAIN.parse::<HeaderValue>().unwrap(),
        config::LOCAL_CORS_DOMAIN.parse::<HeaderValue>().unwrap(),
    ];

    let middleware = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_timeout_error))
        .timeout(Duration::from_secs(*config::SUBMIT_TIME_OUT));

    // register routes
    let router = Router::new()
        .route("/", get(controller::ping))
        .route("/auth/login", post(controller::auth::login))
        .route("/auth/refresh", post(controller::auth::refresh))
        .route("/room/join", post(controller::room::join))
        .route("/team/get-id", get(controller::team::get_id))
        .route(
            "/editor/socket/:question_id/:team_id",
            get(controller::editor_socket),
        )
        .route("/question/get", get(controller::question::get))
        .route(
            "/question/get-by-room",
            get(controller::question::get_by_room),
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
        );

    // add openapi doc and swagger
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origins)
                .allow_headers(ALLOW_HEADERS)
                .expose_headers(ALLOW_HEADERS)
                .allow_credentials(true)
                .allow_methods(ALLOW_METHODS),
        )
        .route_layer(middleware::from_fn(track_metrics));

    // init state
    router.with_state(state)
}
