#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Return pong")
    )
)]
pub async fn ping() -> &'static str {
    "Pong"
}
