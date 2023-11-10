use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use super::ApiState;
use super::Result;

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreateIdentityRequest {
  email: String,
  login: String,
  password: String,
}

#[derive(Serialize, ToSchema)]
pub(crate) struct CreateIdentityResponse {
  access_token: String,
}

/// Create identity
#[utoipa::path(
  post,
  path = "/api/identity",
  operation_id = "create_identity",
  tag = "Identity endpoint",
  request_body = CreateIdentityRequest,
  responses(
    (status = 200, description= "Access Token", body = CreateIdentityResponse),       
  )
)]
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
