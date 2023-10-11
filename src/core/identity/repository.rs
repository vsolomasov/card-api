use async_trait::async_trait;

use super::domain::{Identity, IdentityLogin};
use super::Result;

#[async_trait]
pub trait Repository {
  async fn first_by_login(&self, login: IdentityLogin) -> Result<Identity>;
}
