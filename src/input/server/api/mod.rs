mod handler;

use super::error::{Error, Result};
use crate::{
  core::identity::repository::Repository as IdentityRepository, input::config::ServerConfig,
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

pub async fn server<R>(config: ServerConfig, repo: Arc<R>) -> Result<()>
where
  R: IdentityRepository + Send + Sync + 'static,
{
  let raw_addr = format!("{}:{}", &config.host, &config.port);
  let addr = raw_addr
    .parse::<SocketAddr>()
    .unwrap_or_else(|_| panic!("Error parsing addr {}", raw_addr));

  info!("api server is listening {}:{}", &config.host, &config.port);

  axum::Server::bind(&addr)
    .serve(handler::routes(repo).into_make_service())
    .await
    .map_err(|err| Error::Hyper(err))
}
