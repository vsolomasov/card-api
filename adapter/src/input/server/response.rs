use serde::Serialize;

use super::error::ClientError;
use super::middleware::RequestId;

// region: -- EmptyResponse
#[derive(Serialize)]
pub struct EmptyResponse {
  request_id: RequestId,
}

impl EmptyResponse {
  pub fn new(request_id: &RequestId) -> EmptyResponse {
    EmptyResponse {
      request_id: request_id.clone(),
    }
  }
}
// endregion: -- EmptyResponse

// region: -- ResponseWith
#[derive(Serialize)]
pub struct ResponseWith<P> {
  request_id: RequestId,
  payload: P,
}

impl<P: Serialize> ResponseWith<P> {
  pub fn new(request_id: &RequestId, payload: P) -> ResponseWith<P> {
    ResponseWith {
      request_id: request_id.clone(),
      payload,
    }
  }
}
// endregion: -- ResponseWith

// region: -- ErrorPayload
#[derive(Serialize)]
pub struct ErrorPayload {
  request_id: RequestId,
  client_error: String,
}

impl ErrorPayload {
  pub fn create(request_id: &RequestId, client_error: &ClientError) -> Self {
    ErrorPayload {
      request_id: request_id.clone(),
      client_error: client_error.as_ref().to_string(),
    }
  }
}
// endregion: -- ErrorPayload
