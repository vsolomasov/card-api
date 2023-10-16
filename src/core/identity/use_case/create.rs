use std::sync::Arc;

use super::Result;
use crate::core::identity::domain::IdentityEmail;
use crate::core::identity::domain::IdentityId;
use crate::core::identity::domain::IdentityLogin;
use crate::core::identity::repository::Repository as IdentityRepository;
use crate::core::identity::service;

pub async fn execute<R>(repo: Arc<R>, email: String, login: String) -> Result<IdentityId>
where R: IdentityRepository {
  let identity_email = IdentityEmail::try_from(email)?;
  let identity_login = IdentityLogin::try_from(login)?;

  service::is_email_unique(Arc::clone(&repo), &identity_email).await?;
  service::is_login_unique(Arc::clone(&repo), &identity_login).await?;
  repo.create(&identity_login, &identity_email).await
}
