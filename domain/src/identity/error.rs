use crate::crypt::Error as CryptError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  // IdentityPasswordIsDifferent,
  IdentityPasswordIsEmpty,
  IdentityLoginIsEmpty,
  IdentityEmailIsEmpty,
  IdentityByEmailNotFound(String),
  IdentityByLoginNotFound(String),
  EmailAlreadyExists(String),
  LoginAlreadyExists(String),
  Crypt(CryptError),
  Repository(String),
}

impl From<CryptError> for Error {
  fn from(value: CryptError) -> Self {
    Error::Crypt(value)
  }
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}
