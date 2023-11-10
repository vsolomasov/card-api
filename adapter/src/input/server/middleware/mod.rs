pub(super) mod auth;
pub(super) mod id;

use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

pub use self::auth::auth_middleware;
pub use self::auth::Auth;
pub use self::id::id_middleware;
pub use self::id::RequestId;
use super::Error;
use super::Result;
use crate::input::server::response::ErrorPayload;

pub async fn response_middleware<P>(req: Request<P>, next: Next<P>) -> Result<Response> {
  let res = next.run(req).await;

  let service_error = res.extensions().get::<Error>();
  let client_status_error = service_error.map(|se| se.client_status_and_error());

  let error_response = client_status_error.as_ref().map(|(status, client_error)| {
    let body = ErrorPayload::create(client_error);
    (*status, Json(body)).into_response()
  });

  Ok(error_response.unwrap_or(res))
}
