use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::app_state::AppState;
use crate::database::model::Account;
use crate::Error;
use crate::Result;

use super::TokenPair;

#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginData {
    #[validate(email)]
    email: String,
    password: String,
}

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
    Json(login_data): Json<LoginData>,
) -> Result<Json<TokenPair>> {
    login_data.validate().map_err(anyhow::Error::from)?;

    let account = Account::get_one_by_email(&login_data.email, &state.database).await?;
    if !account.is_enabled || account.is_locked {
        return Err(Error::Forbidden {
            message: format!("Account with email {} cannot be used now", login_data.email),
        });
    }

    if account.is_logged_in {
        return Err(Error::Forbidden {
            message: "Cannot login. Account is logged in other device!".to_string(),
        });
    }

    let is_password_valid = bcrypt::verify(login_data.password, &account.password.unwrap())
        .map_err(|error| anyhow::Error::from(error))?;
    if !is_password_valid {
        return Err(Error::Other(anyhow::anyhow!("Invalid password")));
    }

    sqlx::query!(
        r#"
            UPDATE accounts
            SET is_logged_in = true
            WHERE accounts.id = $1
        "#,
        account.id
    )
    .execute(&state.database)
    .await;

    let token_pair = TokenPair::new(account.id)?;

    Ok(Json(token_pair))
}
