use std::sync::Arc;

use super::Result;
use crate::core::identity::use_case::create;
use crate::core::{ctx::Ctx, identity::repository::Repository as IdentityRepository};
use crate::input::server::response::ResponseWith;
use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn routes<R>(repo: Arc<R>) -> Router
where
  R: IdentityRepository + Send + Sync + 'static,
{
  Router::new()
    .route("/", post(create_handle))
    .with_state(Arc::clone(&repo))
}

// region: -- CreateHandle
#[derive(Deserialize)]
struct CreateIdentityRequest {
  email: String,
  login: String,
}

#[derive(Serialize)]
struct CreateIdentityResponse {
  id: Uuid,
}

async fn create_handle<R>(
  State(repository): State<Arc<R>>,
  ctx: Ctx,
  Json(request_body): Json<CreateIdentityRequest>,
) -> Result<Json<ResponseWith<CreateIdentityResponse>>>
where
  R: IdentityRepository,
{
  let identity_id = create::execute::<R>(
    Arc::clone(&repository),
    request_body.email,
    request_body.login,
  )
  .await?;

  let response_body = ResponseWith::new(
    &ctx,
    CreateIdentityResponse {
      id: identity_id.raw().to_owned(),
    },
  );
  Ok(Json(response_body))
}
// endregion: -- CreateHandle
