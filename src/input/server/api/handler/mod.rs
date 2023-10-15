mod identity;

pub use super::{Error, Result};
use axum::Router;

pub fn routes() -> Router {
  Router::new().nest("/identity", identity::routes())
}
