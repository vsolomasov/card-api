const URL: &str = "http://localhost:8081";

#[tokio::test]
async fn test_system_liveness() -> httpc_test::Result<()> {
  let client = httpc_test::new_client(URL).unwrap();

  client
    .do_get("/system/liveness")
    .await
    .unwrap()
    .print()
    .await
    .unwrap();

  Ok(())
}

#[tokio::test]
async fn test_system_readiness() -> httpc_test::Result<()> {
  let client = httpc_test::new_client(URL).unwrap();

  client
    .do_get("/system/readiness")
    .await
    .unwrap()
    .print()
    .await
    .unwrap();

  Ok(())
}
