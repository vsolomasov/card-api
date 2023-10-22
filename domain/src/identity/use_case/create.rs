use std::sync::Arc;

use super::Result;
use crate::identity::domain::IdentityDecryptedPassword;
use crate::identity::domain::IdentityEmail;
use crate::identity::domain::IdentityEncryptedPassword;
use crate::identity::domain::IdentityForCreate;
use crate::identity::domain::IdentityId;
use crate::identity::domain::IdentityLogin;
use crate::identity::domain::IdentityPasswordSalt;
use crate::identity::repository::Repository as IdentityRepository;
use crate::identity::service;

pub async fn execute(
  repository: Arc<dyn IdentityRepository>,
  passwod_key: &str,
  email: String,
  login: String,
  password: String,
) -> Result<IdentityId> {
  let identity_email = IdentityEmail::try_from(email)?;
  let identity_login = IdentityLogin::try_from(login)?;
  let identity_decrypted_password = IdentityDecryptedPassword::try_from(password)?;

  service::is_email_unique(Arc::clone(&repository), &identity_email).await?;
  service::is_login_unique(Arc::clone(&repository), &identity_login).await?;

  let identity_password_salt: IdentityPasswordSalt = IdentityPasswordSalt::create();
  let identity_encrypted_password = IdentityEncryptedPassword::create(
    &identity_decrypted_password,
    &identity_password_salt,
    passwod_key,
  )?;

  let identity_for_create = IdentityForCreate {
    login: identity_login,
    email: identity_email,
    salt: identity_password_salt,
    password: identity_encrypted_password,
  };

  repository.create(&identity_for_create).await
}
