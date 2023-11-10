pub(crate) mod error;

use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::extract::TypedHeader;
use axum::headers::authorization::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use domain::identity::domain::Identity;

use self::error::Error;
use crate::input::server::api::ApiState;
use crate::input::server::Error as ServerError;
use crate::input::server::Result as ServerResult;

#[derive(Clone)]
pub struct Auth(Identity);

impl Auth {
  pub fn identity(&self) -> &Identity {
    &self.0
  }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Auth {
  type Rejection = ServerError;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> ServerResult<Self> {
    parts
      .extensions
      .get::<Auth>()
      .ok_or(ServerError::AuthMiddleware(Error::AuthNotFound))
      .map(|auth| auth.clone())
  }
}

pub async fn auth_middleware<P>(
  State(api_state): State<Arc<ApiState>>,
  req: Request<P>,
  next: Next<P>,
) -> ServerResult<Response> {
  let (mut parts, body) = req.into_parts();

  let TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>> = parts
    .extract()
    .await
    .map_err(|_| Error::BearerTokenNotFound)?;

  let identity = api_state
    .identity_usecase
    .authorization
    .execute(bearer.token())
    .await
    .map_err(|crypt_error| Error::DecodeError(crypt_error.to_string()))?;

  let mut new_req = Request::from_parts(parts, body);
  new_req.extensions_mut().insert(Auth(identity));
  Ok(next.run(new_req).await)
}
