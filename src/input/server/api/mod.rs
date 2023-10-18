mod handler;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::middleware;
use axum::Router;
use tracing::info;

use super::error::Error;
use super::error::Result;
use crate::input::config::ServerConfig;
use crate::input::server::middleware::ctx_middleware;
use crate::input::server::middleware::response_middleware;
use crate::output::repository::SqlRepository;

pub async fn server(config: ServerConfig, repo: Arc<SqlRepository>) -> Result<()> {
  let raw_addr = format!("{}:{}", &config.host, &config.port);
  let addr = raw_addr
    .parse::<SocketAddr>()
    .unwrap_or_else(|_| panic!("Error parsing addr {}", raw_addr));
  let routes = Router::new()
    .nest("/api", handler::routes(Arc::clone(&repo)))
    .layer(middleware::from_fn(response_middleware))
    .layer(middleware::from_fn(ctx_middleware));

  info!("api server is listening {}:{}", &config.host, &config.port);

  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await
    .map_err(|err| Error::Hyper(err))
}
