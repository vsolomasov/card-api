use std::sync::Arc;

use axum::extract::State;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::ApiState;
use super::Result;
use crate::input::server::middleware::auth_middleware;
use crate::input::server::middleware::Auth;
use crate::input::server::middleware::RequestId;
use crate::input::server::response::ResponseWith;

pub fn routes(state: Arc<ApiState>) -> Router {
  let auth_layer = middleware::from_fn_with_state(Arc::clone(&state), auth_middleware);

  Router::new()
    .route("/login", post(login_handle))
    .route("/auth", get(auth_handle))
    .layer(auth_layer)
    .route("/", post(create_handle))
    .with_state(state)
}

// region: LoginHandle
#[derive(Deserialize)]
struct LoginIdentityRequest {
  email_or_login: String,
  password: String,
}

#[derive(Serialize)]
struct LoginIdentityResponse {
  access_token: String,
}

async fn login_handle(
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
// endregion

// region: AuthHandle
#[derive(Serialize)]
struct AuthResponse {
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

async fn auth_handle(
  request_id: RequestId,
  auth: Auth,
) -> Result<Json<ResponseWith<AuthResponse>>> {
  let payload = AuthResponse::from(auth);
  let response_body = ResponseWith::new(&request_id, payload);
  Ok(Json(response_body))
}
// endregion

// region: CreateHandle
#[derive(Deserialize)]
struct CreateIdentityRequest {
  email: String,
  login: String,
  password: String,
}

#[derive(Serialize)]
struct CreateIdentityResponse {
  access_token: String,
}

async fn create_handle(
  State(api_state): State<Arc<ApiState>>,
  request_id: RequestId,
  Json(request_body): Json<CreateIdentityRequest>,
) -> Result<Json<ResponseWith<CreateIdentityResponse>>> {
  let access_token = api_state
    .identity_usecase
    .create
    .execute(
      request_body.email,
      request_body.login,
      request_body.password,
    )
    .await?;

  let response_body = ResponseWith::new(&request_id, CreateIdentityResponse { access_token });
  Ok(Json(response_body))
}
// endregion
