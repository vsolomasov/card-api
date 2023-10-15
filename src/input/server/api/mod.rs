mod error;
mod handler;

pub use error::{Error, Result};

use crate::input::config::ServerConfig;
use std::net::SocketAddr;
use tracing::info;

pub async fn server(config: ServerConfig) -> Result<()> {
  let raw_addr = format!("{}:{}", &config.host, &config.port);
  let addr = raw_addr
    .parse::<SocketAddr>()
    .unwrap_or_else(|_| panic!("Error parsing addr {}", raw_addr));

  info!("api server is listening {}:{}", &config.host, &config.port);

  axum::Server::bind(&addr)
    .serve(handler::routes().into_make_service())
    .await
    .map_err(|err| Error::Hyper(err))
}
