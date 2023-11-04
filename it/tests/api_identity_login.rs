use hyper::StatusCode;
use it::input::server;
use it::input::server::api::identity::CreateRequest;
use it::input::server::api::identity::IdentityClient;
use it::input::server::api::identity::LoginRequest;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;

#[tokio::test]
#[ignore]
async fn test_api_identity_login_by_login() {
  let client = server::HttpClient::create();
  let random_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

  let email = format!("{}_email", &random_str);
  let login = format!("{}_login", &random_str);

  let create_request_body = CreateRequest {
    email: email,
    login: login.clone(),
    password: random_str.clone(),
  };

  let login_request_body = LoginRequest {
    email_or_login: login,
    password: random_str,
  };

  IdentityClient::create(&client, create_request_body)
    .await
    .unwrap();

  let login_response = IdentityClient::login(&client, login_request_body)
    .await
    .unwrap();

  assert_eq!(login_response.status, StatusCode::OK);
  assert!(!login_response.body.access_token.is_empty());
}

#[tokio::test]
#[ignore]
async fn test_api_identity_login_by_email() {
  let client = server::HttpClient::create();
  let random_str: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

  let email = format!("{}_email", &random_str);
  let login = format!("{}_login", &random_str);

  let create_request_body = CreateRequest {
    email: email.clone(),
    login: login,
    password: random_str.clone(),
  };

  let login_request_body = LoginRequest {
    email_or_login: email,
    password: random_str,
  };

  IdentityClient::create(&client, create_request_body)
    .await
    .unwrap();

  let login_response = IdentityClient::login(&client, login_request_body)
    .await
    .unwrap();

  assert_eq!(login_response.status, StatusCode::OK);
  assert!(!login_response.body.access_token.is_empty());
}
