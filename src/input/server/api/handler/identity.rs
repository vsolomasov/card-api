use super::Result;
use crate::input::server::response::EmptyResponse;
use axum::{routing::get, Json, Router};

pub fn routes() -> Router {
  Router::new().route("/", get(create_handle))
}

async fn create_handle() -> Result<Json<EmptyResponse>> {
  let response = EmptyResponse::new();
  Ok(Json(response))
}
