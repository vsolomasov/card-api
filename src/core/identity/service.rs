use super::{
  domain::{IdentityEmail, IdentityId, IdentityLogin},
  repository::Repository,
  Error, Result,
};

async fn is_email_unique(repo: &dyn Repository, email: &IdentityEmail) -> Result<()> {
  let by_email_res = repo.first_by_email(&email).await;

  match by_email_res {
    Ok(_) => Err(Error::IdentityByEmailAlreadyExists(email.raw().to_owned())),
    Err(Error::IdentityByEmailNotFound(_)) => Ok(()),
    Err(err) => Err(err),
  }
}

async fn is_login_unique(repo: &dyn Repository, login: &IdentityLogin) -> Result<()> {
  let by_login_res = repo.first_by_login(&login).await;

  match by_login_res {
    Ok(_) => Err(Error::IdentityByLoginAlreadyExists(login.raw().to_owned())),
    Err(Error::IdentityByLoginNotFound(_)) => Ok(()),
    Err(err) => Err(err),
  }
}

pub async fn create(
  repo: &dyn Repository,
  email: &IdentityEmail,
  login: &IdentityLogin,
) -> Result<IdentityId> {
  is_email_unique(repo, email).await?;
  is_login_unique(repo, login).await?;
  repo.create(login, email).await
}
