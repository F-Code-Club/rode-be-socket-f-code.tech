use std::sync::Arc;

use axum::extract::State;

use crate::{
    api::extractor::JWTClaims, app_state::AppState, database::model::Account, Error, Result,
};

#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/logout",
    responses (
        (status = Status::OK, description = "Successfully logout!"),
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
pub async fn logout(State(state): State<Arc<AppState>>, jwt_claims: JWTClaims) -> Result<()> {
    let account = Account::get_one_by_id(jwt_claims.sub, &state.database)
        .await
        .map_err(|err| Error::Unauthorized {
            message: err.to_string(),
        })?;

    let logout_result = logout_internal(state, account).await?;
    Ok(logout_result)
}

async fn logout_internal(state: Arc<AppState>, account: Account) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        UPDATE accounts
        SET is_logged_in = false
        WHERE accounts.id = $1
        "#,
        account.id
    )
    .execute(&state.database)
    .await?;

    Ok(())
}
