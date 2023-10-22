use super::parse_env;
use super::Result;

// region: -- SecretConfig
#[derive(Clone)]
pub struct SecretConfig {
  pub password_key: String,
}

impl SecretConfig {
  pub fn load_from_env() -> Result<SecretConfig> {
    Ok(Self {
      password_key: parse_env("SECRET_PWD_KEY")?,
    })
  }
}
// endregion: -- SecretConfig

// region: -- test
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
    let config = SecretConfig::load_from_env()?;
    assert_eq!(
      "db672edbb595ca4b46832b6b0b1a4175a3b5a57d5dbfbd283e6530336610e52a",
      config.password_key
    );
    Ok(())
  }
}
// endregion: -- test
