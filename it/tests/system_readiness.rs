use hyper::StatusCode;
use it::input::server::SystemClient;
use it::input::server::{self};

#[tokio::test]
#[ignore]
async fn test_system_readiness() {
  let client = server::HttpClient::create();
  let response = SystemClient::readiness_probe(&client).await.unwrap();

  assert_eq!(response.status, StatusCode::OK);
  assert_eq!(response.body.status, "Ready");
}
