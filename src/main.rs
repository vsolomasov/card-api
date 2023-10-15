mod core;
mod input;
mod output;

use input::{
  config::ServerConfig,
  server::{self, Status},
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  let config = input::config::Config::load().unwrap();
  let status = Arc::new(Mutex::new(Status::NotReady));

  let mut servers = Vec::new();
  system_server_start(config.server.system, Arc::clone(&status), &mut servers).await;
  api_server_start(config.server.api, Arc::clone(&status), &mut servers).await;

  for server in servers {
    tokio::join!(server).0.unwrap().unwrap();
  }
}

async fn api_server_start(
  config: ServerConfig,
  status: Arc<Mutex<Status>>,
  servers: &mut Vec<tokio::task::JoinHandle<Result<(), server::Error>>>,
) {
  let system_server = server::api_server(config);
  servers.push(tokio::spawn(system_server));
  *status.lock().unwrap() = Status::Ready
}

async fn system_server_start(
  config: ServerConfig,
  status: Arc<Mutex<Status>>,
  servers: &mut Vec<tokio::task::JoinHandle<Result<(), server::Error>>>,
) {
  let system_server = server::system_server(config, Arc::clone(&status));
  servers.push(tokio::spawn(system_server))
}
