use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use domain::identity::Error as IdentityError;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
  SERVICE_ERROR,
  CONFLICT,
}

#[derive(Debug)]
pub enum Error {
  Identity(IdentityError),
  CtxNotFound,
  Axum(String),
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    match self {
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

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    debug!("error {} insert into response", self);
    let mut placeholder = StatusCode::INTERNAL_SERVER_ERROR.into_response();
    placeholder.extensions_mut().insert(self);
    placeholder
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
