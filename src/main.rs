mod core;
mod input;
mod output;

use input::server::system::{self, Status};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  let config = input::config::Config::load().unwrap();

  let status = Arc::new(Mutex::new(system::Status::NotReady));
  let system_server = system::server(config.server.system, Arc::clone(&status));

  let mut servers = Vec::new();
  servers.push(tokio::spawn(system_server));

  *status.lock().unwrap() = Status::Ready;
  for server in servers {
    tokio::join!(server).0.unwrap().unwrap();
  }
}
