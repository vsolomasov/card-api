use super::parse_env;
use super::Result;

// region: SecretConfig
#[derive(Clone)]
pub struct SecretConfig {
  pub password_key: String,
  pub jwt_key: String,
  pub jwt_expiration_sec: i64,
}

impl SecretConfig {
  pub fn load_from_env() -> Result<SecretConfig> {
    Ok(Self {
      password_key: parse_env("SECRET_PWD_KEY")?,
      jwt_key: parse_env("SECRET_JWT_KEY")?,
      jwt_expiration_sec: parse_env("SECRET_JWT_EXPIRATION_SEC")?,
    })
  }
}
// endregion

// region: test
#[cfg(test)]
mod test {
  use std::env;

  use anyhow::Result;

  use super::*;

  #[test]
  fn test_input_config_secret() -> Result<()> {
    env::set_var(
      "SECRET_PWD_KEY",
      "db672edbb595ca4b46832b6b0b1a4175a3b5a57d5dbfbd283e6530336610e52a",
    );
    env::set_var(
      "SECRET_JWT_KEY",
      "a991c65a7344665c1a51e93b935e0cdba1b8372ad08cd993d00f1ec3e258f34d",
    );
    env::set_var("SECRET_JWT_EXPIRATION_SEC", "111");

    let config = SecretConfig::load_from_env()?;

    assert_eq!(
      "db672edbb595ca4b46832b6b0b1a4175a3b5a57d5dbfbd283e6530336610e52a",
      config.password_key
    );
    assert_eq!(
      "a991c65a7344665c1a51e93b935e0cdba1b8372ad08cd993d00f1ec3e258f34d",
      config.jwt_key
    );
    assert_eq!(111, config.jwt_expiration_sec);

    Ok(())
  }
}
// endregion
