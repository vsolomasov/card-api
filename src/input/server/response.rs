use serde::Serialize;
use uuid::Uuid;
use crate::core::ctx::Ctx;

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
