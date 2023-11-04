use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use super::ApiState;
use super::Result;

#[derive(Deserialize)]
pub(super) struct CreateIdentityRequest {
  email: String,
  login: String,
  password: String,
}

#[derive(Serialize)]
pub(super) struct CreateIdentityResponse {
  access_token: String,
}

pub(super) async fn handle(
  State(api_state): State<Arc<ApiState>>,
  Json(request_body): Json<CreateIdentityRequest>,
) -> Result<Json<CreateIdentityResponse>> {
  let access_token = api_state
    .identity_usecase
    .create
    .execute(
      request_body.email,
      request_body.login,
      request_body.password,
    )
    .await?;

  let response_body = CreateIdentityResponse { access_token };
  Ok(Json(response_body))
}
