use std::sync::Arc;

use async_trait::async_trait;

use super::domain::Identity;
use super::domain::IdentityEmail;
use super::domain::IdentityLogin;
use super::Error;
use super::IdentitySecret;
use super::Result;
use crate::crypt::jwt_encode;
use crate::crypt::JwtPayload;
use crate::identity::repository::IdentityRepository;

pub struct IdentityServiceImpl {
  pub identity_repository: Arc<dyn IdentityRepository>,
  pub identity_secret: IdentitySecret,
}

#[async_trait]
pub trait IdentityService: Send + Sync {
  async fn is_email_unique(&self, email: &IdentityEmail) -> Result<()>;
  async fn is_login_unique(&self, login: &IdentityLogin) -> Result<()>;
  fn create_token(&self, identity: Identity) -> Result<String>;
}

#[async_trait]
impl IdentityService for IdentityServiceImpl {
  async fn is_email_unique(&self, email: &IdentityEmail) -> Result<()> {
    let by_email_res = self.identity_repository.first_by_email(&email).await;

    match by_email_res {
      Ok(_) => Err(Error::EmailAlreadyExists(email.value().to_owned())),
      Err(Error::IdentityByEmailNotFound(_)) => Ok(()),
      Err(err) => Err(err),
    }
  }

  async fn is_login_unique(&self, login: &IdentityLogin) -> Result<()> {
    let by_login_res = self.identity_repository.first_by_login(&login).await;

    match by_login_res {
      Ok(_) => Err(Error::LoginAlreadyExists(login.value().to_owned())),
      Err(Error::IdentityByLoginNotFound(_)) => Ok(()),
      Err(err) => Err(err),
    }
  }

  fn create_token(&self, identity: Identity) -> Result<String> {
    let jwt_payload = JwtPayload {
      id: identity.id.value().to_owned(),
      login: identity.login.value().to_owned(),
      email: identity.email.value().to_owned(),
    };

    let access_token = jwt_encode(
      jwt_payload,
      &self.identity_secret.jwt_key.as_bytes(),
      self.identity_secret.jwt_expiration_sec,
    )?;

    Ok(access_token)
  }
}
