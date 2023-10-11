mod core;
mod error;
mod input;
mod output;

#[tokio::main]
async fn main() {
  let config = input::config::Config::load().unwrap();
  println!("Config: {:?}", config);
}
