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
    database::model::{Account, Member, Room},
    enums::AccountRole,
    error::Error,
    util::team::JoinResult,
    Result,
};

#[derive(Debug, Deserialize)]
pub struct JoinRoomInfo {
    #[serde(rename = "roomId")]
    room_id: String,
    #[serde(rename = "code")]
    room_code: Option<String>,
}

#[debug_handler]
pub async fn join(
    jwt_claims: JWTClaims,
    State(state): State<Arc<AppState>>,
    Json(join_room_info): Json<JoinRoomInfo>,
) -> Result<Json<JoinResult>> {
    let account = Account::get_one_by_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        })?;

    let join_result = join_internal(account, jwt_claims, join_room_info, state).await?;

    Ok(join_result)
}

async fn join_internal(
    account: Account,
    jwt_claims: JWTClaims,
    join_room_info: JoinRoomInfo,
    state: Arc<AppState>,
) -> anyhow::Result<Json<JoinResult>> {
    anyhow::ensure!(
        account.role != AccountRole::User,
        "Only user can join a user room!"
    );

    let member: Member = Member::get_one_by_account_id(jwt_claims.sub, &state.database).await?;
    anyhow::ensure!(member.has_join_room, "You have joined a room!");

    let room: Room = sqlx::query_as("SELECT * FROM rooms WHERE id = ?")
        .bind(&join_room_info.room_id)
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
        anyhow::ensure!(now < room.open_time, "Room has not been opened yet!");
        anyhow::ensure!(now > room.open_time, "Room has been closed!");
    }

    sqlx::query(
        r#"
        UPDATE members
           SET has_join_room = true
           WHERE members.id = ?
        "#,
    )
    .bind(&account.id)
    .execute(&state.database)
    .await?;

    Ok(Json(JoinResult {
        room_id: room.id.to_string(),
    }))
}
