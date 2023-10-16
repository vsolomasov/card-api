use std::sync::Arc;

use super::domain::IdentityEmail;
use super::domain::IdentityLogin;
use super::repository::Repository;
use super::Error;
use super::Result;

pub async fn is_email_unique<R>(repo: Arc<R>, email: &IdentityEmail) -> Result<()>
where R: Repository {
  let by_email_res = repo.first_by_email(&email).await;

  match by_email_res {
    Ok(_) => Err(Error::EmailAlreadyExists(email.raw().to_owned())),
    Err(Error::IdentityByEmailNotFound(_)) => Ok(()),
    Err(err) => Err(err),
  }
}

pub async fn is_login_unique<R>(repo: Arc<R>, login: &IdentityLogin) -> Result<()>
where R: Repository {
  let by_login_res = repo.first_by_login(&login).await;

  match by_login_res {
    Ok(_) => Err(Error::LoginAlreadyExists(login.raw().to_owned())),
    Err(Error::IdentityByLoginNotFound(_)) => Ok(()),
    Err(err) => Err(err),
  }
}
