use hyper::StatusCode;
use it::input::server;
use it::input::server::api::identity::CreateRequest;
use it::input::server::api::identity::IdentityClient;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;

#[tokio::test]
#[ignore]
async fn test_api_identity_create() {
  let client = server::HttpClient::create();
  let random_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

  let request_body = CreateRequest {
    email: format!("{}_email", &random_str),
    login: format!("{}_login", &random_str),
    password: random_str,
  };
  let response = IdentityClient::create(&client, request_body).await.unwrap();

  assert_eq!(response.status, StatusCode::OK);
  assert!(!response.body.request_id.is_empty());
  assert!(!response.body.payload.access_token.is_empty());
}
