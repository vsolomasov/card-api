mod core;
mod input;
mod output;

use std::sync::Arc;
use std::sync::Mutex;

use self::input::server;
use self::input::server::Status;
use self::output::repository::SqlRepository;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  let config = input::config::Config::load().unwrap();
  let status = Arc::new(Mutex::new(Status::NotReady));

  let mut servers = Vec::new();

  let system_server = server::system_server(config.server.system, Arc::clone(&status));
  servers.push(tokio::spawn(system_server));

  let repository = SqlRepository::create(&config.repository).await.unwrap();
  let system_server = server::api_server(config.server.api, Arc::new(repository));
  servers.push(tokio::spawn(system_server));
  *status.lock().unwrap() = Status::Ready;

  for server in servers {
    tokio::join!(server).0.unwrap().unwrap();
  }
}
