use serde::Serialize;
use uuid::Uuid;

// region: -- EmptyResponse
#[derive(Serialize)]
pub struct EmptyResponse {
  pub tracking_id: Uuid,
}

impl EmptyResponse {
  pub fn new() -> EmptyResponse {
    EmptyResponse {
      tracking_id: Uuid::new_v4(),
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
  pub fn new(payload: P) -> ResponseWith<P> {
    ResponseWith {
      tracking_id: Uuid::new_v4(),
      payload,
    }
  }
}
// endregion: -- ResponseWith
