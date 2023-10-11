mod error;

use std::{env, str::FromStr};

pub use error::{Error, Result};
use tracing::info;

// region: -- Config
#[derive(Debug)]
pub struct Config {
  pub server: ServerConfig,
  pub repository: RepositoryConfig,
}

impl Config {
  pub fn load() -> Result<Self> {
    info!("config is loading");
    Ok(Self {
      server: ServerConfig::load_from_env()?,
      repository: RepositoryConfig::load_from_env()?,
    })
  }
}
// endregion: -- Config

// region: -- ServerConfig
#[derive(Debug)]
pub struct ServerConfig {
  pub host: String,
  pub port: u32,
}

impl ServerConfig {
  fn load_from_env() -> Result<Self> {
    Ok(Self {
      host: parse_env("SERVER_HOST")?,
      port: parse_env("SERVER_PORT")?,
    })
  }
}
// endregion: -- ServerConfig

// region: -- RepositoryConfig
#[derive(Debug)]
pub struct RepositoryConfig {
  pub host: String,
  pub port: u32,
  pub user: String,
  pub password: String,
  pub database: String,
  pub pool: u32,
}

impl RepositoryConfig {
  fn load_from_env() -> Result<Self> {
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
// endregion: RepositoryConfig

// region: -- private func
fn get_env(name: &'static str) -> Result<String> {
  env::var(name).map_err(|_| Error::ConfigIsMissing(name))
}

fn parse_env<V: FromStr>(name: &'static str) -> Result<V> {
  get_env(name).and_then(|env| env.parse::<V>().map_err(|_| Error::ConfigWrongFormat(name)))
}
// endregion: -- private func

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_config_load_from_env_ok() -> Result<()> {
    env::set_var("SERVER_HOST", "0.0.0.0");
    env::set_var("SERVER_PORT", "80");
    env::set_var("REPOSITORY_HOST", "1.1.1.1");
    env::set_var("REPOSITORY_PORT", "88");
    env::set_var("REPOSITORY_USER", "user");
    env::set_var("REPOSITORY_PASSWORD", "password");
    env::set_var("REPOSITORY_DATABASE", "database");
    env::set_var("REPOSITORY_POOL", "123");

    let config = Config::load()?;

    assert_eq!("0.0.0.0", config.server.host);
    assert_eq!(80, config.server.port);
    assert_eq!("1.1.1.1", config.repository.host);
    assert_eq!(88, config.repository.port);
    assert_eq!("user", config.repository.user);
    assert_eq!("password", config.repository.password);
    assert_eq!("database", config.repository.database);
    assert_eq!(123, config.repository.pool);

    Ok(())
  }

  #[test]
  fn test_config_load_from_env_err() -> Result<()> {
    env::remove_var("SERVER_HOST");

    let actual_error = Config::load().unwrap_err();
    let expected_error = Error::ConfigIsMissing("SERVER_HOST");

    assert_eq!(expected_error, actual_error);

    Ok(())
  }
}
