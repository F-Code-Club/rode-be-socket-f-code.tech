use std::sync::Arc;

use axum::extract::State;
use utoipa::ToSchema;

use crate::api::extractor::JWTClaims;
use crate::app_state::AppState;
use crate::database::model::Member;
use crate::error::{Error, Result};
use crate::util::scoring::css::render_diff_image;

#[derive(ToSchema)]
pub struct RenderDiffParam {
    question_image_buffer: &'static [u8],
    html: String,
}

#[utoipa::path (
    post,
    tag = "Scoring",
    path = "/scoring/css",
    request_body = RenderDiffParam,
    responses (
        (status = 200, description = "Scoring successfully!",body = ExecutionResult),
        (status = 400, description = "Bad request!"),
        (status = 401, description = "User's token is not authorized or missed!")
    )
)]
pub async fn render_diff(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    render_diff_param: RenderDiffParam,
) -> Result<(f32, Vec<u8>)> {
    let _ = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        });

    Ok(render_diff_image(
        render_diff_param.question_image_buffer,
        render_diff_param.html,
    )
    .await?)
}
