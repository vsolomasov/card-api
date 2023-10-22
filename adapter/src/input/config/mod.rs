mod app;
mod error;
mod repository;
mod secret;
mod server;

use std::env;
use std::str::FromStr;

pub use self::app::AppConfig;
pub use self::error::Error;
pub use self::error::Result;
pub use self::repository::RepositoryConfig;
pub use self::secret::SecretConfig;
pub use self::server::ServerConfig;
pub use self::server::ServersConfig;

pub fn load() -> Result<AppConfig> {
  AppConfig::load_from_env()
}

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
