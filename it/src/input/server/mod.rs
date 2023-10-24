mod system;

use hyper::client::HttpConnector;
use hyper::Client;
use hyper::StatusCode;
pub use system::SystemClient;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub struct Response<B> {
  pub status: StatusCode,
  pub body: B,
}

pub struct HttpClient {
  client: Client<HttpConnector>,
}

impl HttpClient {
  pub fn create() -> Self {
    Self {
      client: Client::new(),
    }
  }
}
