use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Method;
use axum::http::Request;
use axum::http::Uri;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Serialize;
use time::OffsetDateTime;
use tracing::info;
use tracing::span;
use tracing::Level;
use uuid::Uuid;

use super::Error;
use super::Result;
use crate::input::server::response::ErrorPayload;

#[derive(Serialize, Clone)]
pub struct RequestId(Uuid);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for RequestId {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    parts
      .extensions
      .get::<RequestId>()
      .ok_or(Error::RequestId)
      .map(|request_id| request_id.clone())
  }
}

pub async fn response_middleware<P>(
  request_id: RequestId,
  req: Request<P>,
  next: Next<P>,
) -> Result<Response> {
  let res = next.run(req).await;

  let service_error = res.extensions().get::<Error>();
  let client_status_error = service_error.map(|se| se.client_status_and_error());

  let error_response = client_status_error.as_ref().map(|(status, client_error)| {
    let body = ErrorPayload::create(&request_id, client_error);
    (*status, Json(body)).into_response()
  });

  Ok(error_response.unwrap_or(res))
}

pub async fn ctx_middleware<P>(
  uri: Uri,
  method: Method,
  mut req: Request<P>,
  next: Next<P>,
) -> Result<Response> {
  let start_time = OffsetDateTime::now_utc().unix_timestamp_nanos();
  let request_id = Uuid::new_v4();

  req.extensions_mut().insert(RequestId(request_id));

  let span =
    span!(Level::INFO, "ctx_middleware", request_id = %request_id, uri = %uri, method = %method);
  let _span = span.enter();

  info!("request received");
  let res = next.run(req).await;

  let end_time = OffsetDateTime::now_utc().unix_timestamp_nanos();
  let code = res.status().as_u16();
  let request_time = ((end_time - start_time) / 1_000_000) as i64;
  info!(code, request_time, "request handled");

  Ok(res)
}
