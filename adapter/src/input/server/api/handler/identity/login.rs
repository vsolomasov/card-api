use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use super::ApiState;
use super::Result;
use crate::input::server::middleware::RequestId;
use crate::input::server::response::ResponseWith;

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
  request_id: RequestId,
  Json(request_body): Json<LoginIdentityRequest>,
) -> Result<Json<ResponseWith<LoginIdentityResponse>>> {
  let access_token = api_state
    .identity_usecase
    .login
    .execute(request_body.email_or_login, request_body.password)
    .await?;

  let response_body = ResponseWith::new(&request_id, LoginIdentityResponse { access_token });
  Ok(Json(response_body))
}
