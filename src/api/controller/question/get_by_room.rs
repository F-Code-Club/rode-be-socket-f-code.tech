use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::Result;

async fn get_by_room_internal(
    room_code: String,
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
        room_code
    )
    .fetch_all(database)
    .await?;

    Ok(Json(question_ids))
}

#[utoipa::path(
    get,
    path = "/question/get-by-room",
    tag = "Question",
    request_body(
        content = String,
        description = "Room code"
    ),
    responses (
        (status = StatusCode::OK, description = "Question ids of the room", body = Vec<Uuid>),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    ),
)]
pub async fn get_by_room(
    State(state): State<Arc<AppState>>,
    room_code: String,
) -> Result<Json<Vec<Uuid>>> {
    let question_ids = get_by_room_internal(room_code, &state.database).await?;

    Ok(question_ids)
}
