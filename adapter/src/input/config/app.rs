use super::RepositoryConfig;
use super::Result;
use super::SecretConfig;
use super::ServersConfig;

// region: AppConfig
pub struct AppConfig {
  pub server: ServersConfig,
  pub repository: RepositoryConfig,
  pub secret: SecretConfig,
}

impl AppConfig {
  pub fn load_from_env() -> Result<Self> {
    Ok(Self {
      server: ServersConfig::load_from_env()?,
      repository: RepositoryConfig::load_from_env()?,
      secret: SecretConfig::load_from_env()?,
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
  fn test_input_config_app() -> Result<()> {
    env::set_var("SYSTEM_SERVER_HOST", "1.1.1.1");
    env::set_var("SYSTEM_SERVER_PORT", "111");
    env::set_var("API_SERVER_HOST", "2.2.2.2");
    env::set_var("API_SERVER_PORT", "222");
    env::set_var("REPOSITORY_HOST", "3.3.3.3");
    env::set_var("REPOSITORY_PORT", "333");
    env::set_var("REPOSITORY_USER", "user");
    env::set_var("REPOSITORY_PASSWORD", "password");
    env::set_var("REPOSITORY_DATABASE", "database");
    env::set_var("REPOSITORY_POOL", "444");
    env::set_var(
      "SECRET_PWD_KEY",
      "e85e648bc0dece079d39bece0d1fb280635ebdc6e09668e7e55386897b6a6271",
    );
    env::set_var(
      "SECRET_JWT_KEY",
      "1167efd4e93a2632cd7f054589ee37d087f7d21228b41bcee839d80146a78225",
    );
    env::set_var("SECRET_JWT_EXPIRATION_SEC", "555");

    let config = AppConfig::load_from_env()?;

    assert_eq!("1.1.1.1", config.server.system.host);
    assert_eq!(111, config.server.system.port);
    assert_eq!("2.2.2.2", config.server.api.host);
    assert_eq!(222, config.server.api.port);
    assert_eq!("3.3.3.3", config.repository.host);
    assert_eq!(333, config.repository.port);
    assert_eq!("user", config.repository.user);
    assert_eq!("password", config.repository.password);
    assert_eq!("database", config.repository.database);
    assert_eq!(444, config.repository.pool);
    assert_eq!(
      "e85e648bc0dece079d39bece0d1fb280635ebdc6e09668e7e55386897b6a6271",
      config.secret.password_key
    );
    assert_eq!(
      "1167efd4e93a2632cd7f054589ee37d087f7d21228b41bcee839d80146a78225",
      config.secret.jwt_key
    );
    assert_eq!(555, config.secret.jwt_expiration_sec);

    Ok(())
  }
}
// endregion
