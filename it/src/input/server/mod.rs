pub mod api;
mod system;

use hyper::client::HttpConnector;
use hyper::Body;
use hyper::Client;
use hyper::StatusCode;
pub use system::SystemClient;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

fn check_request_id(response: &hyper::Response<Body>) -> &str {
  response
    .headers()
    .get("x-request-id")
    .map(|hv| hv.to_str().expect("x-request-id header is not a string"))
    .expect("x-request-id header is missing")
}

pub struct Response<B> {
  pub status: StatusCode,
  pub body: B,
}

pub struct EmptyResponse {
  pub status: StatusCode,
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
