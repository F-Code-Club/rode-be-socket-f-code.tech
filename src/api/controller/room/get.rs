use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::app_state::AppState;
use crate::database::model;
use crate::Result;

#[derive(Deserialize, IntoParams)]
pub struct GetRoomData {
    room_code: String,
}

#[derive(Serialize, ToSchema)]
pub struct Room {
    #[schema(value_type = String)]
    pub open_time: NaiveDateTime,
    #[schema(value_type = String)]
    pub close_time: NaiveDateTime,
}

#[utoipa::path(
    get,
    path = "/room/get",
    tag = "Room",
    params(
        GetRoomData
    ),
    responses (
        (status = StatusCode::OK, description = "Requested room's data", body = Room),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    ),
)]
pub async fn get(
    State(state): State<Arc<AppState>>,
    Query(data): Query<GetRoomData>,
) -> Result<Json<Room>> {
    let question = get_internal(state, data).await?;

    Ok(question)
}

async fn get_internal(state: Arc<AppState>, data: GetRoomData) -> anyhow::Result<Json<Room>> {
    let room = model::Room::get_one_by_code(data.room_code, &state.database).await?;

    Ok(Json(Room {
        open_time: room.open_time,
        close_time: room.close_time,
    }))
}
