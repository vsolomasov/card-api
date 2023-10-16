mod identity;

use super::Result;
use crate::core::identity::repository::Repository as IdentityRepository;
use axum::Router;
use std::sync::Arc;

pub fn routes<T>(repo: Arc<T>) -> Router
where
  T: IdentityRepository + Send + Sync + 'static,
{
  Router::new().nest("/identity", identity::routes::<T>(repo))
}
