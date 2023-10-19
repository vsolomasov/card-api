use std::sync::Arc;
use std::sync::Mutex;

use adapter::input::config::Config;
use adapter::input::config::RepositoryConfig;
use adapter::input::config::ServerConfig;
use adapter::input::server;
use adapter::input::server::Status;
use adapter::output::repository::SqlRepository;
use tokio::task::JoinHandle;
use tracing::error;

#[tokio::main]
async fn main() {
  adapter::output::tracing::init();

  let config = Config::load().unwrap();
  let status = Arc::new(Mutex::new(Status::NotReady));
  let system_server_jh = start_system_server(config.server.system, Arc::clone(&status)).await;
  let api_server_jh =
    start_api_server(config.server.api, config.repository, Arc::clone(&status)).await;

  if let Err(_) = tokio::join!(api_server_jh).0 {
    error!("api server crashed");
    system_server_jh.abort();
  }
}

async fn start_api_server(
  server_config: ServerConfig,
  repository_config: RepositoryConfig,
  status: Arc<Mutex<Status>>,
) -> JoinHandle<()> {
  let repository = SqlRepository::create(&repository_config).await.unwrap();
  let server = server::api_server(server_config, Arc::new(repository));
  let join_handle = tokio::spawn(async { server.await.unwrap() });

  {
    let mut status = status.lock().unwrap();
    *status = Status::Ready;
  }

  join_handle
}

async fn start_system_server(
  server_config: ServerConfig,
  status: Arc<Mutex<Status>>,
) -> JoinHandle<()> {
  let server = server::system_server(server_config, status);
  tokio::spawn(async { server.await.unwrap() })
}
