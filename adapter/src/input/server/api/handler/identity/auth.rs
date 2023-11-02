use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use super::Result;
use crate::input::server::middleware::Auth;
use crate::input::server::middleware::RequestId;
use crate::input::server::response::ResponseWith;

#[derive(Serialize)]
pub(super) struct AuthResponse {
  id: Uuid,
  login: String,
  email: String,
}

impl From<Auth> for AuthResponse {
  fn from(value: Auth) -> Self {
    let identity = value.identity();
    Self {
      id: identity.id.value().clone(),
      login: identity.login.value().clone(),
      email: identity.email.value().clone(),
    }
  }
}

pub(super) async fn handle(
  request_id: RequestId,
  auth: Auth,
) -> Result<Json<ResponseWith<AuthResponse>>> {
  let payload = AuthResponse::from(auth);
  let response_body = ResponseWith::new(&request_id, payload);
  Ok(Json(response_body))
}
