use std::sync::Arc;

use async_trait::async_trait;

use super::domain::IdentityEmail;
use super::domain::IdentityLogin;
use super::Error;
use super::Result;
use crate::identity::repository::IdentityRepository;

pub struct IdentityServiceImpl {
  pub identity_repository: Arc<dyn IdentityRepository>,
}

#[async_trait]
pub trait IdentityService: Send + Sync {
  async fn is_email_unique(&self, email: &IdentityEmail) -> Result<()>;
  async fn is_login_unique(&self, login: &IdentityLogin) -> Result<()>;
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
}
