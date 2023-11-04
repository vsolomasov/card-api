mod identity;
mod openapi;

use std::sync::Arc;

use axum::Router;

pub(super) use self::openapi::ApiDoc;
use super::ApiState;
use super::Result;

pub fn routes(state: Arc<ApiState>) -> Router {
  Router::new().nest("/identity", identity::routes(state))
}
