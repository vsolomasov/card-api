use jsonwebtoken::decode;
use jsonwebtoken::encode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::Error;
use super::Result;

#[cfg_attr(test, derive(Debug, Clone))]
pub struct JwtPayload {
  pub id: Uuid,
  pub login: String,
  pub email: String,
}

#[derive(Serialize, Deserialize)]
struct Claim {
  exp: i64,
  id: Uuid,
  login: String,
  email: String,
}

pub fn jwt_encode(payload: JwtPayload, key: &[u8], expiration: i64) -> Result<String> {
  let claim = Claim {
    exp: expiration,
    id: payload.id,
    login: payload.login,
    email: payload.email,
  };

  let key = EncodingKey::from_secret(key);
  encode(&Header::default(), &claim, &key).map_err(|err| Error::FailToEncodeJwt(err.to_string()))
}

pub fn jwt_decode(token: &String, key: &[u8]) -> Result<JwtPayload> {
  let key = DecodingKey::from_secret(key);
  decode::<Claim>(&token, &key, &Validation::default())
    .map(|token| JwtPayload {
      id: token.claims.id,
      login: token.claims.login,
      email: token.claims.email,
    })
    .map_err(|err| Error::FailToDecodeJwt(err.to_string()))
}

#[cfg(test)]
mod tests {
  use anyhow::Result;
  use time::Duration;
  use time::OffsetDateTime;
  use uuid::Uuid;

  use super::*;

  #[test]
  fn test_crypt_token_jwt() -> Result<()> {
    let key = "53cr3t".as_bytes();
    let expiration = (OffsetDateTime::now_utc() + Duration::minutes(2)).unix_timestamp();
    let payload = JwtPayload {
      id: Uuid::new_v4(),
      login: "login".to_string(),
      email: "email".to_string(),
    };

    let token = jwt_encode(payload.clone(), key, expiration)?;
    let claim = jwt_decode(&token, key)?;

    assert_eq!(claim.id, payload.id);
    assert_eq!(claim.login, payload.login);
    assert_eq!(claim.email, payload.email);

    Ok(())
  }

  #[test]
  fn test_crypt_token_jwt_err() -> Result<()> {
    let key = "53cr3t".as_bytes();
    let expiration = (OffsetDateTime::now_utc() - Duration::minutes(2)).unix_timestamp();
    let payload = JwtPayload {
      id: Uuid::new_v4(),
      login: "login".to_string(),
      email: "email".to_string(),
    };

    let expected_error = Error::FailToDecodeJwt("ExpiredSignature".to_string());
    let token = jwt_encode(payload, key, expiration)?;
    let actual_error = jwt_decode(&token, key).unwrap_err();

    assert_eq!(actual_error, expected_error);
    Ok(())
  }
}
