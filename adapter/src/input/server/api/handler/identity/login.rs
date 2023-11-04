use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use super::ApiState;
use super::Result;

#[derive(Deserialize)]
pub(super) struct LoginIdentityRequest {
  email_or_login: String,
  password: String,
}

#[derive(Serialize)]
pub(super) struct LoginIdentityResponse {
  access_token: String,
}

pub(super) async fn handle(
  State(api_state): State<Arc<ApiState>>,
  Json(request_body): Json<LoginIdentityRequest>,
) -> Result<Json<LoginIdentityResponse>> {
  let access_token = api_state
    .identity_usecase
    .login
    .execute(request_body.email_or_login, request_body.password)
    .await?;

  let response_body = LoginIdentityResponse { access_token };
  Ok(Json(response_body))
}
