use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use domain::identity::Error as IdentityError;
use tracing::debug;

use crate::input::server::middleware::auth::error::Error as AuthError;
use crate::input::server::middleware::id::error::Error as IdError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
  SERVICE_ERROR,
  CONFLICT,
  UNAUTHORIZED,
}

#[derive(Debug)]
pub enum Error {
  IdMiddleware(IdError),
  AuthMiddleware(AuthError),
  Identity(IdentityError),
  Axum(String),
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    match self {
      Self::IdMiddleware(id) => id.client_status_and_error(),
      Self::AuthMiddleware(auth) => auth.client_status_and_error(),
      Self::Identity(IdentityError::EmailAlreadyExists(_))
      | Self::Identity(IdentityError::LoginAlreadyExists(_)) => {
        (StatusCode::CONFLICT, ClientError::CONFLICT)
      }
      _ => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ClientError::SERVICE_ERROR,
      ),
    }
  }
}

impl From<IdError> for Error {
  fn from(error: IdError) -> Self {
    Self::IdMiddleware(error)
  }
}

impl From<AuthError> for Error {
  fn from(error: AuthError) -> Self {
    Self::AuthMiddleware(error)
  }
}

impl From<IdentityError> for Error {
  fn from(error: IdentityError) -> Self {
    Self::Identity(error)
  }
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    debug!("error {} insert into response", self);
    let mut placeholder = StatusCode::INTERNAL_SERVER_ERROR.into_response();
    placeholder.extensions_mut().insert(self);
    placeholder
  }
}
