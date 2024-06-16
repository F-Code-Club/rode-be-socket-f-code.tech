use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{app_state::AppState, database::model, Result};

#[derive(Deserialize, IntoParams)]
pub struct GetQuestionData {
    question_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct QuestionData {
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
    params(
        GetQuestionData
    ),
    responses (
        (status = StatusCode::OK, description = "requested question's data", body = Question),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    ),
)]
pub async fn get(
    State(state): State<Arc<AppState>>,
    Query(data): Query<GetQuestionData>,
) -> Result<Json<QuestionData>> {
    let question = get_internal(state, data).await?;

    Ok(question)
}

async fn get_internal(
    state: Arc<AppState>,
    data: GetQuestionData,
) -> anyhow::Result<Json<QuestionData>> {
    let question = model::Question::get_one_by_id(data.question_id, &state.database).await?;
    let template =
        model::Template::get_one_by_question_id(data.question_id, &state.database).await?;

    Ok(Json(QuestionData { question, template }))
}
