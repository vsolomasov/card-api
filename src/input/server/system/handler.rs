use super::{Result, Status};
use crate::{core::ctx::Ctx, input::server::response::EmptyResponse};
use axum::{
  extract::State,
  response::{IntoResponse, Response},
  routing::get,
  Json, Router,
};
use hyper::StatusCode;
use std::sync::{Arc, Mutex};

pub fn routes(status: Arc<Mutex<Status>>) -> Router {
  Router::new()
    .route("/liveness", get(liveness_handler))
    .route("/readiness", get(readiness_handler))
    .with_state(status)
}

async fn liveness_handler(ctx: Ctx) -> Result<Json<EmptyResponse>> {
  let response = EmptyResponse::new(&ctx);
  Ok(Json(response))
}

async fn readiness_handler(
  State(status_arc): State<Arc<Mutex<Status>>>,
  ctx: Ctx,
) -> Result<Response> {
  let status = *status_arc.lock().unwrap();
  let body = EmptyResponse::new(&ctx);
  let mut response = (StatusCode::SERVICE_UNAVAILABLE, Json(&body)).into_response();

  if let Status::Ready = status {
    response = (StatusCode::OK, Json(&body)).into_response();
  }

  Ok(response)
}
