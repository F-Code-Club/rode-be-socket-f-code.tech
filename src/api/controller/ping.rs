#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = StatusCode::OK, description = "Return pong", body = &'static str)
    )
)]
/// Used to test if the server is running
pub async fn ping() -> &'static str {
    "Pong"
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use rstest::rstest;
    use tower::ServiceExt;

    use crate::api::router;
    use crate::app_state::AppState;

    #[rstest]
    #[trace]
    #[tokio::test]
    async fn return_pong() {
        let state = Arc::new(AppState::new().await.unwrap());
        let app = router::build(state);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Pong");
    }
}
