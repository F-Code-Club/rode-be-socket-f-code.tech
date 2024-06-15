use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app_state::AppState, database::model, Result};

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
    request_body(
        content = Uuid,
        description = "Question id"
    ),
    responses (
        (status = StatusCode::OK, description = "requested question's data", body = Question),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    ),
)]
pub async fn get(
    State(state): State<Arc<AppState>>,
    Json(id): Json<Uuid>,
) -> Result<Json<QuestionData>> {
    let question = get_internal(state, id).await?;

    Ok(question)
}

async fn get_internal(state: Arc<AppState>, id: Uuid) -> anyhow::Result<Json<QuestionData>> {
    let question = model::Question::get_one_by_id(id, &state.database).await?;
    let template = model::Template::get_one_by_question_id(id, &state.database).await?;

    Ok(Json(QuestionData { question, template }))
}
