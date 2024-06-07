use axum::extract::State;
use std::sync::Arc;

use crate::api::extractor::JWTClaims;
use crate::database::model::Member;
use crate::{util, AppState};
use crate::{Error, Result};

#[utoipa::path (
    post,
    tag = "Room",
    path = "/room/join",
    request_body(
        content = String,
        description = "Code of the room which user want to join"
    ),
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
    room_code: String,
) -> Result<()> {
    let member: Member = Member::get_one_by_account_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        })?;
    let join_result = join_internal(state, member, room_code).await?;
    Ok(join_result)
}

async fn join_internal(
    state: Arc<AppState>,
    member: Member,
    room_code: String,
) -> anyhow::Result<()> {
    let room = match sqlx::query_unchecked!(
        r#"SELECT rooms.id, rooms.code, rooms.open_time, rooms.close_time, rooms.is_privated
           FROM rooms
           INNER JOIN scores
           ON rooms.id = scores.room_id 
           AND scores.team_id = $1
           WHERE rooms.code = $2"#,
        member.team_id,
        room_code,
    )
    .fetch_optional(&state.database)
    .await?
    {
        Some(room) => room,
        None => anyhow::bail!("Invalid room code"),
    };

    if room.is_privated {
        anyhow::bail!("The room is privated!");
    }

    let now = util::time::now().naive_local();
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
