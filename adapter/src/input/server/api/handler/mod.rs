mod identity;

use std::sync::Arc;

use axum::Router;

use super::ApiState;
use super::Result;

pub fn routes(state: Arc<ApiState>) -> Router {
  Router::new().nest("/identity", identity::routes(state))
}
