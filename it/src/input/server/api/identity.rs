use async_trait::async_trait;
use hyper::body::to_bytes;
use hyper::Body;
use hyper::Method;
use hyper::Request;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_slice;

use super::Result;
use crate::input::server::HttpClient;
use crate::input::server::Response;

const API_IDENTITY_URL: &str = "http://localhost:8080/api/identity";

// region: Create
#[derive(Serialize)]
pub struct CreateRequest {
  pub email: String,
  pub login: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct CreateResponsePayload {
  pub access_token: String,
}

#[derive(Deserialize)]
pub struct CreateResponse {
  pub request_id: String,
  pub payload: CreateResponsePayload,
}
// endregion

// region: Auth
pub struct AuthRequest {
  pub access_token: String,
}

#[derive(Deserialize)]
pub struct AuthResponsePayload {
  pub id: String,
  pub login: String,
  pub email: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
  pub request_id: String,
  pub payload: AuthResponsePayload,
}
// endregion

#[async_trait]
pub trait IdentityClient {
  async fn create(&self, body: CreateRequest) -> Result<Response<CreateResponse>>;
  async fn auth(&self, body: AuthRequest) -> Result<Response<AuthResponse>>;
}

#[async_trait]
impl IdentityClient for HttpClient {
  async fn create(&self, request_body: CreateRequest) -> Result<Response<CreateResponse>> {
    let request_body_bytes = serde_json::to_vec(&request_body)?;
    let request = Request::builder()
      .method(Method::POST)
      .uri(API_IDENTITY_URL)
      .header("Content-Type", "application/json")
      .body(Body::from(request_body_bytes))?;

    let response = self.client.request(request).await?;
    let response_status = &response.status();
    let response_body_bytes = to_bytes(response.into_body()).await?;
    let response_body = from_slice::<CreateResponse>(&response_body_bytes)?;
    Ok(Response {
      status: *response_status,
      body: response_body,
    })
  }

  async fn auth(&self, body: AuthRequest) -> Result<Response<AuthResponse>> {
    let request = Request::builder()
      .method(Method::GET)
      .uri(format!("{}{}", API_IDENTITY_URL, "/auth"))
      .header("X-ACCESS-TOKEN", body.access_token)
      .body(Body::empty())?;

    let response = self.client.request(request).await?;
    let response_status = &response.status();
    let response_body_bytes = to_bytes(response.into_body()).await?;
    let response_body = from_slice::<AuthResponse>(&response_body_bytes)?;
    Ok(Response {
      status: *response_status,
      body: response_body,
    })
  }
}
