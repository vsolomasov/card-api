use super::parse_env;
use super::Result;

// region: RepositoryConfig
pub struct RepositoryConfig {
  pub host: String,
  pub port: u32,
  pub user: String,
  pub password: String,
  pub database: String,
  pub pool: u32,
}

impl RepositoryConfig {
  pub fn load_from_env() -> Result<Self> {
    Ok(Self {
      host: parse_env("REPOSITORY_HOST")?,
      port: parse_env("REPOSITORY_PORT")?,
      user: parse_env("REPOSITORY_USER")?,
      password: parse_env("REPOSITORY_PASSWORD")?,
      database: parse_env("REPOSITORY_DATABASE")?,
      pool: parse_env("REPOSITORY_POOL")?,
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
  fn test_input_config_repository() -> Result<()> {
    env::set_var("REPOSITORY_HOST", "1.1.1.1");
    env::set_var("REPOSITORY_PORT", "111");
    env::set_var("REPOSITORY_USER", "first_user");
    env::set_var("REPOSITORY_PASSWORD", "first_password");
    env::set_var("REPOSITORY_DATABASE", "first_database");
    env::set_var("REPOSITORY_POOL", "222");

    let config = RepositoryConfig::load_from_env()?;

    assert_eq!("1.1.1.1", config.host);
    assert_eq!(111, config.port);
    assert_eq!("first_user", config.user);
    assert_eq!("first_password", config.password);
    assert_eq!("first_database", config.database);
    assert_eq!(222, config.pool);

    Ok(())
  }
}
// endregion
