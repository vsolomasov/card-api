use super::parse_env;
use super::Result;

// region: -- ServersConfig
pub struct ServersConfig {
  pub system: ServerConfig,
  pub api: ServerConfig,
}

impl ServersConfig {
  pub fn load_from_env() -> Result<ServersConfig> {
    Ok(Self {
      system: ServerConfig::load_from_env("SYSTEM")?,
      api: ServerConfig::load_from_env("API")?,
    })
  }
}
// endregion: -- ServersConfig

// region: -- ServerConfig
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

// region: -- test
#[cfg(test)]
mod test {
  use std::env;

  use anyhow::Result;

  use super::*;

  #[test]
  fn test_input_config_server() -> Result<()> {
    env::set_var("SYSTEM_SERVER_HOST", "1.1.1.1");
    env::set_var("SYSTEM_SERVER_PORT", "111");
    env::set_var("API_SERVER_HOST", "2.2.2.2");
    env::set_var("API_SERVER_PORT", "222");

    let config = ServersConfig::load_from_env()?;

    assert_eq!("1.1.1.1", config.system.host);
    assert_eq!(111, config.system.port);
    assert_eq!("2.2.2.2", config.api.host);
    assert_eq!(222, config.api.port);

    Ok(())
  }
}
// endregion: -- test
