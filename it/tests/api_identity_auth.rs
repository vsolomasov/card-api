use hyper::StatusCode;
use it::input::server;
use it::input::server::api::identity::AuthRequest;
use it::input::server::api::identity::CreateRequest;
use it::input::server::api::identity::IdentityClient;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;

#[tokio::test]
#[ignore]
async fn test_api_identity_auth() {
  let client = server::HttpClient::create();
  let random_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

  let email = format!("{}_email", &random_str);
  let login = format!("{}_login", &random_str);

  let create_request_body = CreateRequest {
    email: email.clone(),
    login: login.clone(),
    password: random_str,
  };

  let create_response = IdentityClient::create(&client, create_request_body)
    .await
    .unwrap();

  let auth_request = AuthRequest {
    access_token: create_response.body.access_token,
  };

  let auth_response = IdentityClient::auth(&client, auth_request).await.unwrap();

  assert_eq!(auth_response.status, StatusCode::OK);
  assert!(!auth_response.body.id.is_empty());
  assert_eq!(auth_response.body.login, login);
  assert_eq!(auth_response.body.email, email);
}
