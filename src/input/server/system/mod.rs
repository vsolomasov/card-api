mod error;
mod handler;

use crate::input::config::ServerConfig;
use axum::Router;
use std::{
  net::SocketAddr,
  sync::{Arc, Mutex},
};
use tracing::info;

pub use error::{Error, Result};

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

  let routes = Router::new().nest("/system", handler::routes(status));

  info!(
    "system server is listening {}:{}",
    &config.host, &config.port
  );

  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await
    .map_err(|err| Error::Hyper(err))
}
