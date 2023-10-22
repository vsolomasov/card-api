mod identity;

use std::sync::Arc;

use axum::Router;

use super::ApiState;
use super::Result;

pub fn routes() -> Router<Arc<ApiState>> {
  Router::new().nest("/identity", identity::routes())
}
