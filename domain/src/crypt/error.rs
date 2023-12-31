pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Error {
  FailToEncodeJwt(String),
  FailToDecodeJwt(String),
  FailToDecodeBase64,
  FailHmacKey,
}

impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}
