use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::Member;
use crate::error::{Error, Result};
use crate::util::scoring::css::render_diff_image;

#[derive(Deserialize, ToSchema)]
pub struct RenderDiffParam {
    question_image_buffer: Vec<u8>,
    html: String,
}

#[utoipa::path (
    post,
    tag = "Scoring",
    path = "/scoring/render_diff",
    request_body = RenderDiffParam,
    responses (
        (status = 200, description = "Scoring successfully!"),
        (status = 400, description = "Bad request!"),
        (status = 401, description = "User's token is not authorized or missed!")
    )
)]
pub async fn render_diff(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    render_diff_param: RenderDiffParam,
) -> Result<Json<Vec<u8>>> {
    let _ = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        });

    let diff_render_image = render_diff_image(
        &render_diff_param.question_image_buffer,
        render_diff_param.html,
    )
    .await?
    .1;

    Ok(Json(diff_render_image))
}
