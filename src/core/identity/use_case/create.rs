use super::Result;
use crate::core::identity::{
  domain::{IdentityEmail, IdentityId, IdentityLogin},
  repository::Repository as IdentityRepository,
  service,
};
use std::sync::Arc;

pub async fn execute<R>(repo: Arc<R>, email: String, login: String) -> Result<IdentityId>
where
  R: IdentityRepository,
{
  let identity_email = IdentityEmail::try_from(email)?;
  let identity_login = IdentityLogin::try_from(login)?;

  service::is_email_unique(Arc::clone(&repo), &identity_email).await?;
  service::is_login_unique(Arc::clone(&repo), &identity_login).await?;
  repo.create(&identity_login, &identity_email).await
}
