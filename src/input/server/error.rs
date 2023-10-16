use crate::core::identity::Error as IdentityError;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Hyper(hyper::Error),
  Identity(IdentityError),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
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
