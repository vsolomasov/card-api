use async_trait::async_trait;

use super::domain::Identity;
use super::domain::IdentityEmail;
use super::domain::IdentityId;
use super::domain::IdentityLogin;
use super::Result;

#[async_trait]
pub trait Repository {
  async fn first_by_login(&self, login: &IdentityLogin) -> Result<Identity>;
  async fn first_by_email(&self, email: &IdentityEmail) -> Result<Identity>;
  async fn create(&self, login: &IdentityLogin, email: &IdentityEmail) -> Result<IdentityId>;
}
