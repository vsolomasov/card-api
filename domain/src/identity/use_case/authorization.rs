use super::Result;
use crate::crypt::jwt_decode;
use crate::identity::domain::Identity;
use crate::identity::domain::IdentityEmail;
use crate::identity::domain::IdentityId;
use crate::identity::domain::IdentityLogin;

pub async fn execute(jwt_key: &str, access_token: &str) -> Result<Identity> {
  let jwt_payload = jwt_decode(&access_token, jwt_key.as_bytes())?;

  let identity_id = IdentityId::from(jwt_payload.id);
  let identity_login = IdentityLogin::try_from(jwt_payload.login)?;
  let identity_email = IdentityEmail::try_from(jwt_payload.email)?;

  let identity = Identity {
    id: identity_id,
    login: identity_login,
    email: identity_email,
  };

  Ok(identity)
}
