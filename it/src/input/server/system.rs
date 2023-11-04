use async_trait::async_trait;
use hyper::body::to_bytes;
use hyper::Body;
use hyper::Method;
use hyper::Request;
use serde::Deserialize;
use serde_json::from_slice;

use super::EmptyResponse;
use super::HttpClient;
use super::Response;
use super::Result;

const SYSTEM_URL: &str = "http://localhost:8081";

#[derive(Deserialize)]
pub struct ReadinessResponse {
  pub status: String,
}

#[async_trait]
pub trait SystemClient {
  async fn liveness_probe(&self) -> Result<EmptyResponse>;
  async fn readiness_probe(&self) -> Result<Response<ReadinessResponse>>;
}

#[async_trait]
impl SystemClient for HttpClient {
  async fn liveness_probe(&self) -> Result<EmptyResponse> {
    let request = Request::builder()
      .method(Method::GET)
      .uri(format!("{}{}", SYSTEM_URL, "/system/liveness"))
      .body(Body::empty())?;

    let response = self.client.request(request).await?;
    super::check_request_id(&response);
    let status = response.status();
    Ok(EmptyResponse { status })
  }

  async fn readiness_probe(&self) -> Result<Response<ReadinessResponse>> {
    let request = Request::builder()
      .method(Method::GET)
      .uri(format!("{}{}", SYSTEM_URL, "/system/readiness"))
      .body(Body::empty())?;

    let response = self.client.request(request).await?;
    super::check_request_id(&response);
    let status = &response.status();
    let body_bytes = to_bytes(response.into_body()).await?;
    let body = from_slice::<ReadinessResponse>(&body_bytes)?;
    Ok(Response {
      status: *status,
      body,
    })
  }
}
