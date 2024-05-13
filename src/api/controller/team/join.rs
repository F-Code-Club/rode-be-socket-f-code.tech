#[allow(unused)]
use anyhow::anyhow;
use axum::{debug_handler, extract::State, Json};
use chrono::Local;
use std::sync::Arc;

use serde::Deserialize;
use std::option::Option;

use crate::{
    api::extractor::JWTClaims,
    app_state::AppState,
    database::model::{Member, Room},
    util::team::JoinResult,
    Error, Result,
};

#[derive(Debug, Deserialize)]
pub struct JoinRoomInfo {
    #[serde(rename = "roomId")]
    room_id: i32,
    #[serde(rename = "code")]
    room_code: Option<String>,
}

#[debug_handler]
pub async fn join(
    State(state): State<Arc<AppState>>,
    jwt_claims: JWTClaims,
    Json(join_room_info): Json<JoinRoomInfo>,
) -> Result<Json<JoinResult>> {
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
) -> anyhow::Result<Json<JoinResult>> {
    let room = sqlx::query_as_unchecked!(
        Room,
        "SELECT * FROM rooms WHERE id = $1 AND code = $2",
        join_room_info.room_id,
        join_room_info.room_code
    )
    .fetch_one(&state.database)
    .await?;
    // let has_joined_room = sqlx::query_scalar!(
    //     r#"
    //     SELECT members.has_join_room FROM members
    //     LEFT JOIN accounts ON accounts.id = members.account_id
    //     WHERE members.account_id = $1 AND accounts.role = $2
    // "#,
    //     &account.id,
    //     account.role
    // )
    // .fetch_one(&state.database)
    // .await
    // .map_err(|_| Error::Other(anyhow!("Cannot get MEMBER from database!")))?;
    if room.is_privated {
        let room_code = join_room_info
            .room_code
            .unwrap_or("Missing room code!".to_string());

        anyhow::ensure!(room.code != room_code, "Room code is incorrect!");

        let now = Local::now().naive_local();
        anyhow::ensure!(now >= room.open_time, "Room has not been opened yet!");
        anyhow::ensure!(now < room.close_time, "Room has been closed!");
    }
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

    Ok(Json(JoinResult {
        room_id: room.id.to_string(),
    }))
}
