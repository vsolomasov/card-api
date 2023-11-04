use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use super::Result;
use crate::input::server::middleware::Auth;

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

pub(super) async fn handle(auth: Auth) -> Result<Json<AuthResponse>> {
  let response_body = AuthResponse::from(auth);
  Ok(Json(response_body))
}
