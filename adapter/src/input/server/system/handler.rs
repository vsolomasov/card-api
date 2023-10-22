use std::sync::Arc;
use std::sync::Mutex;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::Json;
use axum::Router;

use super::Result;
use super::Status;
use crate::input::server::middleware::RequestId;
use crate::input::server::response::EmptyResponse;

pub fn routes(status: Arc<Mutex<Status>>) -> Router {
  Router::new()
    .route("/liveness", get(liveness_handler))
    .route("/readiness", get(readiness_handler))
    .with_state(status)
}

async fn liveness_handler(request_id: RequestId) -> Result<Json<EmptyResponse>> {
  let response = EmptyResponse::new(&request_id);
  Ok(Json(response))
}

async fn readiness_handler(
  State(status_arc): State<Arc<Mutex<Status>>>,
  request_id: RequestId,
) -> Result<Response> {
  let status = *status_arc.lock().unwrap();
  let body = EmptyResponse::new(&request_id);
  let mut response = (StatusCode::SERVICE_UNAVAILABLE, Json(&body)).into_response();

  if let Status::Ready = status {
    response = (StatusCode::OK, Json(&body)).into_response();
  }

  Ok(response)
}
