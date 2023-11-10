use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

use super::identity;

#[derive(OpenApi)]
#[openapi(
  info(title = "CardAPI"),
  servers(
    (url = "http://localhost:8080", description = "Local server"),
  ),
  paths(
    identity::auth::handle,
    identity::create::handle,
    identity::login::handle,
  ),
  components(
    schemas(
      identity::auth::AuthResponse,
      identity::create::CreateIdentityRequest,
      identity::create::CreateIdentityResponse,
      identity::login::LoginIdentityRequest,
      identity::login::LoginIdentityResponse,
    )
  ),
  modifiers(&SecurityAddon)
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi.components.as_mut().unwrap();
    components.add_security_scheme(
      "token",
      SecurityScheme::Http(
        HttpBuilder::new()
          .scheme(HttpAuthScheme::Bearer)
          .bearer_format("JWT")
          .build(),
      ),
    )
  }
}
