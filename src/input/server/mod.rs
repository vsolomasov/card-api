mod api;
mod error;
mod middleware;
mod response;
mod system;

pub use error::{Error, Result};
pub use system::Status;

use super::config::ServerConfig;
use crate::core::identity::repository::Repository as IdentityRepository;
use std::sync::{Arc, Mutex};

pub async fn system_server(config: ServerConfig, status: Arc<Mutex<Status>>) -> Result<()> {
  Ok(system::server(config, status).await?)
}

pub async fn api_server<R>(config: ServerConfig, repo: Arc<R>) -> Result<()>
where
  R: IdentityRepository + Send + Sync + 'static,
{
  Ok(api::server(config, repo).await?)
}
