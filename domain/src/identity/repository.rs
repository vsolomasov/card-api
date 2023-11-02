use async_trait::async_trait;

use super::domain::IdentityEmail;
use super::domain::IdentityEntity;
use super::domain::IdentityForCreate;
use super::domain::IdentityId;
use super::domain::IdentityLogin;
use super::Result;

#[async_trait]
pub trait IdentityRepository: Send + Sync {
  async fn first_by_login(&self, login: &IdentityLogin) -> Result<IdentityEntity>;
  async fn first_by_email(&self, email: &IdentityEmail) -> Result<IdentityEntity>;
  async fn create(&self, identity_for_create: &IdentityForCreate) -> Result<IdentityId>;
}
