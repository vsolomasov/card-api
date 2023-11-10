use axum::http::StatusCode;

use crate::input::server::error::ClientError;

#[derive(Debug)]
pub enum Error {
  RequestIdNotFound,
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      ClientError::SERVICE_ERROR,
    )
  }
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}
