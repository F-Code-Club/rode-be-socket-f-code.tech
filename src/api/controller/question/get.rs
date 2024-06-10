use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{app_state::AppState, database::model, Result};

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetQuestionData {
    /// Stack id of the requested question
    stack_id: Uuid,
    /// Id of the requested question
    id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct Question {
    #[schema(inline)]
    #[serde(flatten)]
    pub question: model::Question,
    #[schema(inline)]
    #[serde(flatten)]
    pub template: model::Template,
}

#[utoipa::path(
    get,
    path = "/question/get",
    tag = "Question",
    params(GetQuestionData),
    responses (
        (status = StatusCode::OK, description = "requested question's data", body = Question),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    ),
)]
pub async fn get(
    State(state): State<Arc<AppState>>,
    Query(data): Query<GetQuestionData>,
) -> Result<Json<Question>> {
    let question = get_internal(state, data).await?;

    Ok(question)
}

async fn get_internal(
    state: Arc<AppState>,
    data: GetQuestionData,
) -> anyhow::Result<Json<Question>> {
    let question = model::Question::get_one_by_ids(data.id, data.stack_id, &state.database).await?;
    let template = model::Template::get_one_by_question_id(data.id, &state.database).await?;

    Ok(Json(Question { question, template }))
}
