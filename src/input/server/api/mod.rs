mod handler;

use super::error::{Error, Result};
use crate::core::identity::repository::Repository as IdentityRepository;
use crate::input::config::ServerConfig;
use axum::Router;
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
  let routes = Router::new().nest("/api", handler::routes(Arc::clone(&repo)));

  info!("api server is listening {}:{}", &config.host, &config.port);

  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await
    .map_err(|err| Error::Hyper(err))
}
