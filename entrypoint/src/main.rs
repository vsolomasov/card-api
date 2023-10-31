use std::sync::Arc;
use std::sync::Mutex;

use adapter::input::config;
use adapter::input::config::RepositoryConfig;
use adapter::input::config::SecretConfig;
use adapter::input::config::ServerConfig;
use adapter::input::server;
use adapter::input::server::Status;
use adapter::output::repository::SqlRepository;
use tokio::task::JoinHandle;
use tracing::error;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() {
  let app_name = concat!("card_api-", env!("CARGO_PKG_VERSION")).to_string();

  let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
  let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);

  let subscriber = Registry::default()
    .with(EnvFilter::from_default_env())
    .with(JsonStorageLayer)
    .with(bunyan_formatting_layer);
  tracing::subscriber::set_global_default(subscriber).expect("setting custom subscriber failed");

  let config = config::load().unwrap();
  let status = Arc::new(Mutex::new(Status::NotReady));
  let system_server_jh = start_system_server(config.server.system, Arc::clone(&status)).await;
  let api_server_jh = start_api_server(
    config.server.api,
    config.secret,
    config.repository,
    Arc::clone(&status),
  )
  .await;

  if let Err(_) = tokio::join!(api_server_jh).0 {
    error!("api server crashed");
    system_server_jh.abort();
  }
}

async fn start_api_server(
  server_config: ServerConfig,
  secret_config: SecretConfig,
  repository_config: RepositoryConfig,
  status: Arc<Mutex<Status>>,
) -> JoinHandle<()> {
  let repository = SqlRepository::create(&repository_config).await.unwrap();
  let server = server::api_server(server_config, secret_config, repository);
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
