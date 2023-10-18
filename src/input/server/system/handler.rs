use std::sync::Arc;
use std::sync::Mutex;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::Json;
use axum::Router;
use hyper::StatusCode;

use super::Result;
use super::Status;
use crate::input::server::middleware::CtxWrapper;
use crate::input::server::response::EmptyResponse;

pub fn routes(status: Arc<Mutex<Status>>) -> Router {
  Router::new()
    .route("/liveness", get(liveness_handler))
    .route("/readiness", get(readiness_handler))
    .with_state(status)
}

async fn liveness_handler(CtxWrapper(ctx): CtxWrapper) -> Result<Json<EmptyResponse>> {
  let response = EmptyResponse::new(&ctx);
  Ok(Json(response))
}

async fn readiness_handler(
  State(status_arc): State<Arc<Mutex<Status>>>,
  CtxWrapper(ctx): CtxWrapper,
) -> Result<Response> {
  let status = *status_arc.lock().unwrap();
  let body = EmptyResponse::new(&ctx);
  let mut response = (StatusCode::SERVICE_UNAVAILABLE, Json(&body)).into_response();

  if let Status::Ready = status {
    response = (StatusCode::OK, Json(&body)).into_response();
  }

  Ok(response)
}
