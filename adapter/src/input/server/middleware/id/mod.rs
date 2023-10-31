mod error;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Method;
use axum::http::Request;
use axum::http::Uri;
use axum::middleware::Next;
use axum::response::Response;
use error::Error;
use error::Result;
use serde::Serialize;
use time::OffsetDateTime;
use tracing::info;
use tracing::span;
use tracing::Level;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct RequestId(Uuid);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for RequestId {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    parts
      .extensions
      .get::<RequestId>()
      .ok_or(Error::RequestIdNotFound)
      .map(|request_id| request_id.clone())
  }
}

pub async fn id_middleware<P>(
  uri: Uri,
  method: Method,
  mut req: Request<P>,
  next: Next<P>,
) -> Result<Response> {
  let start_time = OffsetDateTime::now_utc().unix_timestamp_nanos();
  let request_id = Uuid::new_v4();

  req.extensions_mut().insert(RequestId(request_id));

  let span = span!(Level::INFO, "id_middleware", request_id = %request_id, request_path = %uri, request_method = %method);
  let _span = span.enter();

  let res = next.run(req).await;

  let end_time = OffsetDateTime::now_utc().unix_timestamp_nanos();
  let code = res.status().as_u16();
  let request_time_ms = ((end_time - start_time) / 1_000_000) as i64;
  info!(response_code = code, request_time_ms, "Request handled");

  Ok(res)
}
