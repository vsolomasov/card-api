mod identity;

use std::sync::Arc;

use axum::Router;

use super::Result;
use crate::core::identity::repository::Repository as IdentityRepository;

pub fn routes<T>(repo: Arc<T>) -> Router
where T: IdentityRepository + Send + Sync + 'static {
  Router::new().nest("/identity", identity::routes::<T>(repo))
}
