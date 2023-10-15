mod api;
mod error;
mod response;
mod system;

pub use error::{Error, Result};
pub use system::Status;

use super::config::ServerConfig;
use std::sync::{Arc, Mutex};

pub async fn system_server(config: ServerConfig, status: Arc<Mutex<Status>>) -> Result<()> {
  Ok(system::server(config, status).await?)
}

pub async fn api_server(config: ServerConfig) -> Result<()> {
  Ok(api::server(config).await?)
}
