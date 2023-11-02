use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  AuthNotFound,
  HeaderNotFound(&'static str),
  HeaderNotStr(&'static str),
  DecodeError(String),
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    debug!("auth error {} insert into response", self);
    let mut placeholder = StatusCode::INTERNAL_SERVER_ERROR.into_response();
    placeholder.extensions_mut().insert(self);
    placeholder
  }
}