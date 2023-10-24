use hyper::StatusCode;
use it::input::server::SystemClient;
use it::input::server::{self};

#[tokio::test]
#[ignore]
async fn test_system_liveness() {
  let client = server::HttpClient::create();
  let response = SystemClient::liveness_probe(&client).await.unwrap();

  assert_eq!(response.status, StatusCode::OK);
  assert!(!response.body.request_id.is_empty());
}
