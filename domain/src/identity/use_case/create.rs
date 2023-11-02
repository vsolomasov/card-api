use std::sync::Arc;

use async_trait::async_trait;

use super::IdentitySecret;
use super::Result;
use crate::identity::domain::Identity;
use crate::identity::domain::IdentityDecryptedPassword;
use crate::identity::domain::IdentityEmail;
use crate::identity::domain::IdentityEncryptedPassword;
use crate::identity::domain::IdentityForCreate;
use crate::identity::domain::IdentityLogin;
use crate::identity::domain::IdentityPasswordSalt;
use crate::identity::repository::IdentityRepository;
use crate::identity::service::IdentityService;

pub(crate) struct IdentityCreateUsecaseImpl {
  pub identity_repository: Arc<dyn IdentityRepository>,
  pub identity_service: Arc<dyn IdentityService>,
  pub identity_secret: IdentitySecret,
}

#[async_trait]
pub trait IdentityCreateUsecase: Send + Sync {
  async fn execute(&self, email: String, login: String, password: String) -> Result<String>;
}

#[async_trait]
impl IdentityCreateUsecase for IdentityCreateUsecaseImpl {
  async fn execute(&self, email: String, login: String, password: String) -> Result<String> {
    let identity_email = IdentityEmail::try_from(email)?;
    let identity_login = IdentityLogin::try_from(login)?;
    let identity_decrypted_password = IdentityDecryptedPassword::try_from(password)?;

    self
      .identity_service
      .is_email_unique(&identity_email)
      .await?;
    self
      .identity_service
      .is_login_unique(&identity_login)
      .await?;

    let identity_password_salt: IdentityPasswordSalt = IdentityPasswordSalt::create();
    let identity_encrypted_password = IdentityEncryptedPassword::create(
      &identity_decrypted_password,
      &identity_password_salt,
      &self.identity_secret.password_key,
    )?;

    let identity_for_create = IdentityForCreate {
      login: &identity_login,
      email: &identity_email,
      salt: &identity_password_salt,
      password: &identity_encrypted_password,
    };

    let identity_id = self
      .identity_repository
      .create(&identity_for_create)
      .await?;

    let identity = Identity {
      id: identity_id,
      login: identity_login,
      email: identity_email,
    };

    self.identity_service.create_token(identity)
  }
}
