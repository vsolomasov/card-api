use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use tracing::debug;
use tracing::trace;

use super::Error;
use super::Result;
use crate::core::ctx::Ctx;
use crate::input::server::response::ErrorPayload;

pub async fn response_middleware<P>(ctx: Ctx, req: Request<P>, next: Next<P>) -> Result<Response> {
  let res = next.run(req).await;
  debug!("{} response_middleware", ctx.request_id());

  let service_error = res.extensions().get::<Error>();
  let client_status_error = service_error.map(|se| se.client_status_and_error());

  let error_response = client_status_error.as_ref().map(|(status, client_error)| {
    let body = ErrorPayload::create(&ctx, client_error);
    (*status, Json(body)).into_response()
  });

  Ok(error_response.unwrap_or(res))
}

pub async fn ctx_middleware<P>(mut req: Request<P>, next: Next<P>) -> Result<Response> {
  debug!("init ctx for request");
  req.extensions_mut().insert(Ctx::init());
  Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    trace!("extracting ctx from request parts");
    parts
      .extensions
      .get::<Ctx>()
      .ok_or(Error::CtxNotFound)
      .map(|ctx| ctx.clone())
  }
}
