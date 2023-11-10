use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use super::ApiState;
use super::Result;

#[derive(Deserialize, ToSchema)]
pub(crate) struct LoginIdentityRequest {
  email_or_login: String,
  password: String,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct LoginIdentityResponse {
  access_token: String,
}

/// Login identity
#[utoipa::path(
  post,
  path = "/api/identity/login",
  operation_id = "login_identity",
  tag = "Identity endpoint",
  request_body = LoginIdentityRequest,
  responses(
    (status = 200, description= "Access Token", body = LoginIdentityResponse),       
  )
)]
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
