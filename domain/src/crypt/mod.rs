mod b64_url;
mod error;
mod hmac;
mod token;

pub use self::error::Error;
pub use self::error::Result;
pub use self::token::jwt_decode;
pub use self::token::jwt_encode;
pub use self::token::JwtPayload;

pub fn sign_into_b64_url(key: &[u8], salt: &str, content: &str) -> Result<String> {
  let hmac_result = hmac::sign(key, salt, content)?;
  let b64_url = b64_url::encode(&hmac_result);

  Ok(b64_url)
}

#[cfg(test)]
mod tests {
  use anyhow::Result;

  use super::*;

  #[test]
  fn test_crypt_sign_into_b64_url() -> Result<()> {
    let expected =
      "a0_bmwsCXN23tVo40fXm77goBsUAm8Zojm4nzv1LmeFcdo9p0NjK5GX9a4j3sQ4aG1NaNvjwwC3AEDPxiFhnJQ"
        .to_string();

    let key = "53cr3t".as_bytes();
    let salt = "salt";
    let content = "hello_world";

    let res = sign_into_b64_url(key, salt, content)?;
    assert_eq!(res, expected);

    Ok(())
  }
}
