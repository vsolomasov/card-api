use hmac::Hmac;
use hmac::Mac;
use sha2::Sha512;

use super::error::Error;
use super::error::Result;

pub fn sign(key: &[u8], salt: &str, content: &str) -> Result<Vec<u8>> {
  let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::FailHmacKey)?;

  hmac_sha512.update(content.as_bytes());
  hmac_sha512.update(salt.as_bytes());

  let hmac_result = hmac_sha512.finalize();
  let result_bytes = hmac_result.into_bytes();

  Ok(result_bytes.to_vec())
}
