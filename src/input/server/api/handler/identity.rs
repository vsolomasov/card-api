use std::sync::Arc;

use axum::extract::State;
use axum::routing::post;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::Result;
use crate::core::ctx::Ctx;
use crate::core::identity::repository::Repository as IdentityRepository;
use crate::core::identity::use_case::create;
use crate::input::server::response::ResponseWith;

pub fn routes(repo: Arc<dyn IdentityRepository>) -> Router {
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

async fn create_handle(
  State(repository): State<Arc<dyn IdentityRepository>>,
  ctx: Ctx,
  Json(request_body): Json<CreateIdentityRequest>,
) -> Result<Json<ResponseWith<CreateIdentityResponse>>> {
  let identity_id = create::execute(
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
