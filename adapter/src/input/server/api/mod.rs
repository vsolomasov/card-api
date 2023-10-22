mod handler;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::middleware;
use axum::Router;
use tracing::info;

use super::error::Error;
use super::error::Result;
use crate::input::config::SecretConfig;
use crate::input::config::ServerConfig;
use crate::input::server::middleware::ctx_middleware;
use crate::input::server::middleware::response_middleware;
use crate::output::repository::SqlRepository;

pub struct ApiState {
  pub repository: SqlRepository,
  pub secret: SecretConfig,
}

pub async fn server(
  config: ServerConfig,
  secret: SecretConfig,
  repository: SqlRepository,
) -> Result<()> {
  let raw_addr = format!("{}:{}", &config.host, &config.port);
  let addr = raw_addr
    .parse::<SocketAddr>()
    .unwrap_or_else(|_| panic!("Error parsing addr {}", raw_addr));

  let shared_state = Arc::new(ApiState { repository, secret });
  let routes = Router::new()
    .nest("/api", handler::routes())
    .layer(middleware::from_fn(response_middleware))
    .layer(middleware::from_fn(ctx_middleware))
    .with_state(shared_state);

  info!("api server is listening {}:{}", &config.host, &config.port);

  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await
    .map_err(|err| Error::Axum(err.to_string()))
}
