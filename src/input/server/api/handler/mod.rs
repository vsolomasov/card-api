mod identity;

use std::sync::Arc;

use axum::Router;

use super::Result;
use crate::core::identity::repository::Repository as IdentityRepository;

pub fn routes(repo: Arc<dyn IdentityRepository>) -> Router {
  Router::new().nest("/identity", identity::routes(repo))
}
