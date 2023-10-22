use std::sync::Arc;

use axum::extract::State;
use axum::routing::post;
use axum::Json;
use axum::Router;
use domain::identity::use_case::create;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::ApiState;
use super::Result;
use crate::input::server::middleware::RequestId;
use crate::input::server::response::ResponseWith;

pub fn routes() -> Router<Arc<ApiState>> {
  Router::new().route("/", post(create_handle))
}

// region: -- CreateHandle
#[derive(Deserialize)]
struct CreateIdentityRequest {
  email: String,
  login: String,
  password: String,
}

#[derive(Serialize)]
struct CreateIdentityResponse {
  id: Uuid,
}

async fn create_handle(
  State(api_state): State<Arc<ApiState>>,
  request_id: RequestId,
  Json(request_body): Json<CreateIdentityRequest>,
) -> Result<Json<ResponseWith<CreateIdentityResponse>>> {
  let repository = Arc::new(api_state.repository.clone());
  let identity_id = create::execute(
    repository,
    &api_state.secret.password_key,
    request_body.email,
    request_body.login,
    request_body.password,
  )
  .await?;

  let response_body = ResponseWith::new(
    &request_id,
    CreateIdentityResponse {
      id: identity_id.value().to_owned(),
    },
  );
  Ok(Json(response_body))
}
// endregion: -- CreateHandle
