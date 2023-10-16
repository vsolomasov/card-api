pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  IdentityLoginIsEmpty,
  IdentityEmailIsEmpty,
  IdentityByEmailNotFound(String),
  IdentityByLoginNotFound(String),
  EmailAlreadyExists(String),
  LoginAlreadyExists(String),
  Repository(String),
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}
