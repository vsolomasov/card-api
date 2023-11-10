pub(super) mod auth;
pub(super) mod create;
pub(super) mod login;

use std::sync::Arc;

use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

use super::ApiState;
use super::Result;
use crate::input::server::middleware::auth_middleware;

pub fn routes(state: Arc<ApiState>) -> Router {
  let auth_layer = middleware::from_fn_with_state(Arc::clone(&state), auth_middleware);

  Router::new()
    .route("/auth", get(auth::handle))
    .layer(auth_layer)
    .route("/login", post(login::handle))
    .route("/", post(create::handle))
    .with_state(state)
}
