use serde_json::json;

const URL: &str = "http://localhost:8080";

#[tokio::test]
async fn test_api_identity_create() -> httpc_test::Result<()> {
  let client = httpc_test::new_client(URL).unwrap();

  client
    .do_post(
      "/api/identity",
      json!({
        "login": "first_login",
        "email": "first_email"
      }),
    )
    .await
    .unwrap()
    .print()
    .await
    .unwrap();

  Ok(())
}