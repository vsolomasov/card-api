use super::{Error, Result};
use crate::core::ctx::Ctx;
use async_trait::async_trait;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{extract::FromRequestParts, http::request::Parts};
use tracing::{debug, trace};

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
