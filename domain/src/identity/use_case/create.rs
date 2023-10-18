use std::sync::Arc;

use super::Result;
use crate::identity::domain::IdentityEmail;
use crate::identity::domain::IdentityId;
use crate::identity::domain::IdentityLogin;
use crate::identity::repository::Repository as IdentityRepository;
use crate::identity::service;

pub async fn execute(
  repo: Arc<dyn IdentityRepository>,
  email: String,
  login: String,
) -> Result<IdentityId> {
  let identity_email = IdentityEmail::try_from(email)?;
  let identity_login = IdentityLogin::try_from(login)?;

  service::is_email_unique(Arc::clone(&repo), &identity_email).await?;
  service::is_login_unique(Arc::clone(&repo), &identity_login).await?;
  repo.create(&identity_login, &identity_email).await
}
