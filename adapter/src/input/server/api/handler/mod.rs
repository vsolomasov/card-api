mod identity;

use std::sync::Arc;

use axum::Router;

use super::Result;
use crate::output::repository::SqlRepository;

pub fn routes(repo: Arc<SqlRepository>) -> Router {
  Router::new().nest("/identity", identity::routes(repo))
}
