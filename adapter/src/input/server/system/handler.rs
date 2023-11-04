use std::sync::Arc;
use std::sync::Mutex;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::Serialize;
use tracing::instrument;

use super::Result;
use super::Status;

pub fn routes(status: Arc<Mutex<Status>>) -> Router {
  Router::new()
    .route("/liveness", get(liveness_handler))
    .route("/readiness", get(readiness_handler))
    .with_state(status)
}

#[instrument()]
async fn liveness_handler() -> Result<()> {
  Ok(())
}

#[derive(Serialize)]
pub struct ReadinessResponse {
  pub status: Status,
}

async fn readiness_handler(State(status_arc): State<Arc<Mutex<Status>>>) -> Result<Response> {
  let status = *status_arc.lock().unwrap();
  let body = ReadinessResponse { status };
  let mut response = (StatusCode::SERVICE_UNAVAILABLE, Json(&body)).into_response();

  if let Status::Ready = status {
    response = (StatusCode::OK, Json(&body)).into_response();
  }

  Ok(response)
}
