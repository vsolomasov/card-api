mod api;
mod error;
mod middleware;
mod response;
mod system;

use std::sync::Arc;
use std::sync::Mutex;

use self::error::Error;
use self::error::Result;
pub use self::system::Status;
use super::config::ServerConfig;
use crate::output::repository::SqlRepository;

pub async fn system_server(config: ServerConfig, status: Arc<Mutex<Status>>) -> Result<()> {
  Ok(system::server(config, status).await?)
}

pub async fn api_server(config: ServerConfig, repository: Arc<SqlRepository>) -> Result<()> {
  Ok(api::server(config, repository).await?)
}
