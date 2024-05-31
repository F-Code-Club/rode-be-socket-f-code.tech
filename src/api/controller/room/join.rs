use anyhow::anyhow;
use axum::{debug_handler, extract::State, Json};
use chrono::Local;
use std::sync::Arc;
use utoipa::ToSchema;

use serde::Deserialize;

use crate::{
    api::extractor::JWTClaims,
    app_state::AppState,
    database::model::{Member, Room},
    Error, Result,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct JoinRoomInfo {
    room_code: String,
}

#[debug_handler]
#[utoipa::path (
    post,
    tag = "Room",
    path = "/room/join",
    responses (
        (status = Status::OK, description = "Successfully join a room!"),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
        (
            status = StatusCode::UNAUTHORIZED,
            description = "User's token is not authorized or missed!",
            body = ErrorResponse,
            example = json!({"status": 401, "message": "Invalid token", "details": {}})
        )
    ),
    security(("jwt_token" = []))
)]
pub async fn join(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    Json(join_room_info): Json<JoinRoomInfo>,
) -> Result<()> {
    let member: Member = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        })?;
    let join_result = join_internal(state, member, join_room_info).await?;
    Ok(join_result)
}

async fn join_internal(
    state: Arc<AppState>,
    member: Member,
    join_room_info: JoinRoomInfo,
) -> anyhow::Result<()> {
    let room = sqlx::query_as_unchecked!(
        Room,
        r#"SELECT rooms.*
           FROM rooms
           INNER JOIN scores
           ON rooms.id = scores.room_Id 
           AND scores.team_id = $1
           WHERE rooms.code = $2"#,
        member.team_id,
        join_room_info.room_code,
    )
    .fetch_one(&state.database)
    .await?;

    if room.is_privated {
        anyhow::bail!("The room is privated!");
    }

    let now = Local::now().naive_local();
    anyhow::ensure!(now >= room.open_time, "Room has not been opened yet!");
    anyhow::ensure!(now < room.close_time, "Room has been closed!");

    sqlx::query!(
        r#"
           UPDATE members
           SET has_join_room = true
           WHERE members.id = $1
        "#,
        member.id
    )
    .execute(&state.database)
    .await?;

    Ok(())
}
