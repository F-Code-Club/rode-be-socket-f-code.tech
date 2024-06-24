use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::{Member, Template};
use crate::error::{Error, Result};
use crate::util;

#[derive(Deserialize, ToSchema)]
pub struct RenderDiffImageData {
    question_id: Uuid,
    html: String,
}

#[utoipa::path (
    post,
    tag = "Scoring",
    path = "/scoring/render-diff-image",
    request_body = RenderDiffImageData,
    responses (
        (status = StatusCode::OK, description = "Image representing the differences", body = Vec<u8>),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
        (
            status = StatusCode::UNAUTHORIZED,
            description = "User's token is not authorized or missed!",
            body = ErrorResponse,
            example = json!({"status": 401, "message": "Invalid token", "details": {}})
        ),
        (
            status = StatusCode::REQUEST_TIMEOUT,
            body = ErrorResponse,
            example = json!({"status": 408, "message": "Request timed out", "details": {}})
        ),
    ),
    security(("jwt_token" = []))
)]
/// Create an image representing the differences between the rendered html code and the image from question
pub async fn render_diff_image(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    Json(data): Json<RenderDiffImageData>,
) -> Result<Json<Vec<u8>>> {
    Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        })?;

    let diff_image = render_diff_image_internal(state, data).await?;

    Ok(diff_image)
}

pub async fn render_diff_image_internal(
    state: Arc<AppState>,
    data: RenderDiffImageData,
) -> anyhow::Result<Json<Vec<u8>>> {
    let template = Template::get_one_by_question_id(data.question_id, &state.database).await?;
    let template_buffer = template.download().await?;

    let (_, diff_image) =
        util::scoring::frontend::css::render_diff_image(&template_buffer, data.html).await?;

    Ok(Json(diff_image))
}
