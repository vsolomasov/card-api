mod api;
mod error;
mod middleware;
mod response;
mod system;

use std::sync::Arc;
use std::sync::Mutex;

pub use self::error::Error;
pub use self::error::Result;
pub use self::system::Status;
use super::config::ServerConfig;
use crate::core::identity::repository::Repository as IdentityRepository;

pub async fn system_server(config: ServerConfig, status: Arc<Mutex<Status>>) -> Result<()> {
  Ok(system::server(config, status).await?)
}

pub async fn api_server<R>(config: ServerConfig, repo: Arc<R>) -> Result<()>
where R: IdentityRepository + Send + Sync + 'static {
  Ok(api::server(config, repo).await?)
}
