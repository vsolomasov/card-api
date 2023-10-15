use super::SqlRepository;
use crate::core::identity::domain::{Identity, IdentityEmail, IdentityId, IdentityLogin};
use crate::core::identity::repository::Repository as IdentityRepository;
use crate::core::identity::Error as IdentityError;
use crate::core::identity::Result;
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
struct IdentityEntity {
  id: Uuid,
  login: String,
  email: String,
}

impl TryFrom<IdentityEntity> for Identity {
  type Error = IdentityError;

  fn try_from(value: IdentityEntity) -> Result<Self> {
    let id = IdentityId::from(value.id);
    let login = IdentityLogin::try_from(value.login)?;
    let email = IdentityEmail::try_from(value.email)?;
    Ok(Self { id, login, email })
  }
}

#[async_trait]
impl IdentityRepository for SqlRepository {
  async fn first_by_login(&self, login: &IdentityLogin) -> Result<Identity> {
    sqlx::query_as::<_, IdentityEntity>("select id, login, email from identity where login = $1")
      .bind(login.raw())
      .fetch_optional(&self.connection)
      .await
      .map_err(|err| IdentityError::Repository(err.to_string()))
      .and_then(|opt| {
        opt.ok_or(IdentityError::IdentityByLoginNotFound(
          login.raw().to_owned(),
        ))
      })
      .and_then(|ie| Identity::try_from(ie))
  }

  async fn first_by_email(&self, email: &IdentityEmail) -> Result<Identity> {
    sqlx::query_as::<_, IdentityEntity>("select id, login, email from identity where email = $1")
      .bind(email.raw())
      .fetch_optional(&self.connection)
      .await
      .map_err(|err| IdentityError::Repository(err.to_string()))
      .and_then(|opt| {
        opt.ok_or(IdentityError::IdentityByEmailNotFound(
          email.raw().to_owned(),
        ))
      })
      .and_then(|ie| Identity::try_from(ie))
  }

  async fn create(&self, login: &IdentityLogin, email: &IdentityEmail) -> Result<IdentityId> {
    let id = Uuid::new_v4();
    sqlx::query("insert into identity (id, login, email) values ($1, $2, $3)")
      .bind(id)
      .bind(login.raw())
      .bind(email.raw())
      .execute(&self.connection)
      .await
      .map_err(|err| IdentityError::Repository(err.to_string()))?;

    Ok(IdentityId::from(id))
  }
}
