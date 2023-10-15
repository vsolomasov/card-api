use crate::input::server::api::Error as ApiError;
use crate::input::server::system::Error as SystemError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  System(SystemError),
  Api(ApiError),
}

impl From<SystemError> for Error {
  fn from(err: SystemError) -> Self {
    Error::System(err)
  }
}

impl From<ApiError> for Error {
  fn from(err: ApiError) -> Self {
    Error::Api(err)
  }
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}
