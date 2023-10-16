mod handler;

use super::error::{Error, Result};
use crate::input::config::ServerConfig;
use crate::input::server::middleware::ctx_middleware;
use axum::{middleware, Router};
use std::{
  net::SocketAddr,
  sync::{Arc, Mutex},
};
use tracing::info;

#[derive(Clone, Copy)]
pub enum Status {
  Ready,
  NotReady,
}

pub async fn server(config: ServerConfig, status: Arc<Mutex<Status>>) -> Result<()> {
  let raw_addr = format!("{}:{}", &config.host, &config.port);
  let addr = raw_addr
    .parse::<SocketAddr>()
    .unwrap_or_else(|_| panic!("Error parsing addr {}", raw_addr));

  let routes = Router::new()
    .nest("/system", handler::routes(status))
    .layer(middleware::from_fn(ctx_middleware));

  info!(
    "system server is listening {}:{}",
    &config.host, &config.port
  );

  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await
    .map_err(|err| Error::Hyper(err))
}
