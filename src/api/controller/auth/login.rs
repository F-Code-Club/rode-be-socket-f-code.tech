use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::app_state::AppState;
use crate::database::model::Account;
use crate::Error;
use crate::Result;

use super::util::TokenPair;

#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginData {
    #[validate(email)]
    email: String,
    password: String,
}

/// Login route
///
/// # Mechanisms
/// When a user logs in, revoke all existing tokens associated with their account
#[utoipa::path (
    post,
    tag = "Auth",
    path = "/auth/login",
    request_body = LoginData,
    responses (
        (status = StatusCode::OK, description = "Login successfully!", body = TokenPair),
        (status = StatusCode::FORBIDDEN, description = "Trying to login into an account that cannot be used", body = ErrorResponse),
        (status = StatusCode::BAD_REQUEST, description = "Bad request!", body = ErrorResponse),
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(login_data): Json<LoginData>,
) -> Result<Json<TokenPair>> {
    login_data.validate().map_err(anyhow::Error::from)?;

    let account = Account::get_one_by_email(&login_data.email, &state.database).await?;
    if !account.is_usable() {
        return Err(Error::Forbidden {
            message: format!("Account with email {} cannot be used now", login_data.email),
        });
    }

    let is_password_valid = bcrypt::verify(login_data.password, &account.password.unwrap())
        .map_err(anyhow::Error::from)?;
    if !is_password_valid {
        return Err(Error::Other(anyhow::anyhow!("Invalid password")));
    }

    let token_pair = TokenPair::generate(
        account.id,
        user_agent.to_string(),
        &state.account_fingerprints,
    )?;

    Ok(Json(token_pair))
}
