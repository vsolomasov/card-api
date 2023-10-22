use async_trait::async_trait;

use super::domain::Identity;
use super::domain::IdentityEmail;
use super::domain::IdentityForCreate;
use super::domain::IdentityId;
use super::domain::IdentityLogin;
use super::Result;

#[async_trait]
pub trait Repository: Send + Sync {
  async fn first_by_login(&self, login: &IdentityLogin) -> Result<Identity>;
  async fn first_by_email(&self, email: &IdentityEmail) -> Result<Identity>;
  async fn create(&self, identity_for_create: &IdentityForCreate) -> Result<IdentityId>;
}
