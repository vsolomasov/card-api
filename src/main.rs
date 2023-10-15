mod core;
mod input;
mod output;

use input::server::{self, Status};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  let config = input::config::Config::load().unwrap();

  let mut servers = Vec::new();
  let status = Arc::new(Mutex::new(Status::NotReady));

  let system_server = server::system_server(config.server.system, Arc::clone(&status));
  servers.push(tokio::spawn(system_server));

  let api_server = server::api_server(config.server.api);
  servers.push(tokio::spawn(api_server));

  *status.lock().unwrap() = Status::Ready;
  for server in servers {
    tokio::join!(server).0.unwrap().unwrap();
  }
}
