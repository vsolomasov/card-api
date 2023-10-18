use domain::ctx::Ctx;
use serde::Serialize;
use uuid::Uuid;

use super::error::ClientError;

// region: -- EmptyResponse
#[derive(Serialize)]
pub struct EmptyResponse {
  pub tracking_id: Uuid,
}

impl EmptyResponse {
  pub fn new(ctx: &Ctx) -> EmptyResponse {
    EmptyResponse {
      tracking_id: ctx.request_id().clone(),
    }
  }
}
// endregion: -- EmptyResponse

// region: -- ResponseWith
#[derive(Serialize)]
pub struct ResponseWith<P> {
  pub tracking_id: Uuid,
  pub payload: P,
}

impl<P: Serialize> ResponseWith<P> {
  pub fn new(ctx: &Ctx, payload: P) -> ResponseWith<P> {
    ResponseWith {
      tracking_id: ctx.request_id().clone(),
      payload,
    }
  }
}
// endregion: -- ResponseWith

// region: -- ErrorPayload
#[derive(Serialize)]
pub struct ErrorPayload {
  request_id: String,
  client_error: String,
}

impl ErrorPayload {
  pub fn create(ctx: &Ctx, client_error: &ClientError) -> Self {
    ErrorPayload {
      request_id: ctx.request_id().to_string(),
      client_error: client_error.as_ref().to_string(),
    }
  }
}
// endregion: -- ErrorPayload
