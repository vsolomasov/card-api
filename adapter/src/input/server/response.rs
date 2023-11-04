use serde::Serialize;

use super::error::ClientError;

// region: ErrorPayload
#[derive(Serialize)]
pub struct ErrorPayload {
  client_error: String,
}

impl ErrorPayload {
  pub fn create(client_error: &ClientError) -> Self {
    ErrorPayload {
      client_error: client_error.as_ref().to_string(),
    }
  }
}
// endregion
