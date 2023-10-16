use std::sync::Arc;

use super::domain::IdentityEmail;
use super::domain::IdentityLogin;
use super::Error;
use super::Result;
use crate::core::identity::repository::Repository as IdentityRepository;

pub async fn is_email_unique(
  repo: Arc<dyn IdentityRepository>,
  email: &IdentityEmail,
) -> Result<()> {
  let by_email_res = repo.first_by_email(&email).await;

  match by_email_res {
    Ok(_) => Err(Error::EmailAlreadyExists(email.raw().to_owned())),
    Err(Error::IdentityByEmailNotFound(_)) => Ok(()),
    Err(err) => Err(err),
  }
}

pub async fn is_login_unique(
  repo: Arc<dyn IdentityRepository>,
  login: &IdentityLogin,
) -> Result<()> {
  let by_login_res = repo.first_by_login(&login).await;

  match by_login_res {
    Ok(_) => Err(Error::LoginAlreadyExists(login.raw().to_owned())),
    Err(Error::IdentityByLoginNotFound(_)) => Ok(()),
    Err(err) => Err(err),
  }
}
