mod error;

use std::env;
use std::str::FromStr;

use tracing::info;

pub use self::error::Error;
pub use self::error::Result;

// region: -- Config
#[derive(Debug)]
pub struct Config {
  pub server: ServersConfig,
  pub repository: RepositoryConfig,
}

impl Config {
  pub fn load() -> Result<Self> {
    info!("config is loading");
    Ok(Self {
      server: ServersConfig::load_from_env()?,
      repository: RepositoryConfig::load_from_env()?,
    })
  }
}
// endregion: -- Config

// region: -- ServersConfig
#[derive(Debug)]
pub struct ServersConfig {
  pub system: ServerConfig,
  pub api: ServerConfig,
}

impl ServersConfig {
  fn load_from_env() -> Result<ServersConfig> {
    Ok(Self {
      system: ServerConfig::load_from_env("SYSTEM")?,
      api: ServerConfig::load_from_env("API")?,
    })
  }
}
// endregion: -- ServersConfig

// region: -- ServerConfig
#[derive(Debug)]
pub struct ServerConfig {
  pub host: String,
  pub port: u32,
}

impl ServerConfig {
  fn load_from_env(prefix: &str) -> Result<Self> {
    let server_env = format!("{}_SERVER_HOST", prefix);
    let port_env = format!("{}_SERVER_PORT", prefix);
    let res = Self {
      host: parse_env(&server_env)?,
      port: parse_env(&port_env)?,
    };

    Ok(res)
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
fn get_env(name: &str) -> Result<String> {
  env::var(name).map_err(|_| Error::ConfigIsMissing(name.to_owned()))
}

fn parse_env<V: FromStr>(name: &str) -> Result<V> {
  get_env(name).and_then(|env| {
    env
      .parse::<V>()
      .map_err(|_| Error::ConfigWrongFormat(name.to_owned()))
  })
}
// endregion: -- private func

#[cfg(test)]
mod test {
  use anyhow::Result;

  use super::*;

  #[test]
  fn test_config_load_from_env_ok() -> Result<()> {
    env::set_var("SYSTEM_SERVER_HOST", "1.1.1.1");
    env::set_var("SYSTEM_SERVER_PORT", "111");
    env::set_var("API_SERVER_HOST", "2.2.2.2");
    env::set_var("API_SERVER_PORT", "222");
    env::set_var("REPOSITORY_HOST", "3.3.3.3");
    env::set_var("REPOSITORY_PORT", "333");
    env::set_var("REPOSITORY_USER", "user");
    env::set_var("REPOSITORY_PASSWORD", "password");
    env::set_var("REPOSITORY_DATABASE", "database");
    env::set_var("REPOSITORY_POOL", "123");

    let config = Config::load()?;

    assert_eq!("1.1.1.1", config.server.system.host);
    assert_eq!(111, config.server.system.port);
    assert_eq!("2.2.2.2", config.server.api.host);
    assert_eq!(222, config.server.api.port);
    assert_eq!("3.3.3.3", config.repository.host);
    assert_eq!(333, config.repository.port);
    assert_eq!("user", config.repository.user);
    assert_eq!("password", config.repository.password);
    assert_eq!("database", config.repository.database);
    assert_eq!(123, config.repository.pool);

    Ok(())
  }
}
