use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::Result;

#[derive(Deserialize, IntoParams)]
pub struct GetQuestionByRoomData {
    room_code: String,
}

async fn get_by_room_internal(
    data: GetQuestionByRoomData,
    database: &PgPool,
) -> anyhow::Result<Json<Vec<Uuid>>> {
    let question_ids = sqlx::query_scalar!(
        r#"
SELECT questions.id
FROM rooms
INNER JOIN question_stacks ON rooms.stack_id = question_stacks.id
INNER JOIN questions ON rooms.stack_id = questions.stack_id
WHERE rooms.code = $1
        "#,
        data.room_code
    )
    .fetch_all(database)
    .await?;

    Ok(Json(question_ids))
}

#[utoipa::path(
    get,
    path = "/question/get-by-room",
    tag = "Question",
    params(
        GetQuestionByRoomData
    ),
    responses (
        (status = StatusCode::OK, description = "Question ids of the room", body = Vec<Uuid>),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    ),
)]
pub async fn get_by_room(
    State(state): State<Arc<AppState>>,
    Query(data): Query<GetQuestionByRoomData>,
) -> Result<Json<Vec<Uuid>>> {
    let question_ids = get_by_room_internal(data, &state.database).await?;

    Ok(question_ids)
}
