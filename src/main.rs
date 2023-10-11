mod core;
mod input;
mod output;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  let config = input::config::Config::load().unwrap();
  let _repository = output::repository::SqlRepository::create(&config.repository)
    .await
    .unwrap();

  println!("Config: {:?}", config);
}
