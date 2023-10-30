mod error;

use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use domain::identity::domain::Identity;
use domain::identity::use_case::authorization;

use self::error::Error;
use self::error::Result;
use crate::input::server::api::ApiState;

const ACCESS_TOKEN_KEY: &str = "X-ACCESS-TOKEN";

#[derive(Clone)]
pub struct Auth(Identity);

impl Auth {
  pub fn identity(&self) -> &Identity {
    &self.0
  }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Auth {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    parts
      .extensions
      .get::<Auth>()
      .ok_or(Error::AuthNotFound)
      .map(|auth| auth.clone())
  }
}

pub async fn auth_middleware<P>(
  State(api_state): State<Arc<ApiState>>,
  mut req: Request<P>,
  next: Next<P>,
) -> Result<Response> {
  let raw_header = req
    .headers()
    .get(ACCESS_TOKEN_KEY)
    .ok_or(Error::HeaderNotFound(ACCESS_TOKEN_KEY))?;

  let access_token = raw_header
    .to_str()
    .map_err(|_| Error::HeaderNotStr(ACCESS_TOKEN_KEY))?;

  let identity = authorization::execute(&api_state.secret.jwt_key, access_token)
    .await
    .map_err(|crypt_error| Error::DecodeError(crypt_error.to_string()))?;

  req.extensions_mut().insert(Auth(identity));
  Ok(next.run(req).await)
}
