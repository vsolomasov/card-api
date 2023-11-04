use std::sync::Arc;

use async_trait::async_trait;

use super::Error;
use super::IdentitySecret;
use super::Result;
use crate::identity::domain::Identity;
use crate::identity::domain::IdentityDecryptedPassword;
use crate::identity::domain::IdentityEmail;
use crate::identity::domain::IdentityEncryptedPassword;
use crate::identity::domain::IdentityLogin;
use crate::identity::service::IdentityService;
use crate::identity::IdentityRepository;

pub(crate) struct IdentityLoginUsecaseImpl {
  pub identity_repository: Arc<dyn IdentityRepository>,
  pub identity_service: Arc<dyn IdentityService>,
  pub identity_secret: IdentitySecret,
}

#[async_trait]
pub trait IdentityLoginUsecase: Send + Sync {
  async fn execute(&self, email_or_login: String, password: String) -> Result<String>;
}

#[async_trait]
impl IdentityLoginUsecase for IdentityLoginUsecaseImpl {
  async fn execute(&self, login_or_email: String, password: String) -> Result<String> {
    let identity_by_login = move |login: String| async move {
      let login = IdentityLogin::try_from(login)?;
      let result = self.identity_repository.first_by_login(&login).await;

      if result.is_err() {
        tracing::info!("Identity by login {} not found", login.value());
      }

      result
    };

    let identity_by_email = move |email: String| async move {
      let email = IdentityEmail::try_from(email)?;
      let result = self.identity_repository.first_by_email(&email).await;

      if result.is_err() {
        tracing::info!("Identity by email {} not found", email.value());
      }

      result
    };

    let decrypted_password = IdentityDecryptedPassword::try_from(password)?;

    let identity_entity = identity_by_login(login_or_email.clone())
      .await
      .or(identity_by_email(login_or_email.clone()).await)?;

    let encrypted_password = IdentityEncryptedPassword::create(
      &decrypted_password,
      &identity_entity.salt,
      &self.identity_secret.password_key,
    )?;

    if encrypted_password != identity_entity.password {
      return Err(Error::IdentityPasswordIsDifferent);
    }

    self
      .identity_service
      .create_token(Identity::from(identity_entity))
  }
}
