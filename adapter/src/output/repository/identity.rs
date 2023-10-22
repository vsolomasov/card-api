use async_trait::async_trait;
use domain::identity::domain::Identity;
use domain::identity::domain::IdentityEmail;
use domain::identity::domain::IdentityForCreate;
use domain::identity::domain::IdentityId;
use domain::identity::domain::IdentityLogin;
use domain::identity::repository::Repository as IdentityRepository;
use domain::identity::Error as IdentityError;
use domain::identity::Result;
use sqlx::FromRow;
use uuid::Uuid;

use super::SqlRepository;

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
      .bind(login.value())
      .fetch_optional(&self.connection)
      .await
      .map_err(|err| IdentityError::Repository(err.to_string()))
      .and_then(|opt| {
        opt.ok_or(IdentityError::IdentityByLoginNotFound(
          login.value().to_owned(),
        ))
      })
      .and_then(|ie| Identity::try_from(ie))
  }

  async fn first_by_email(&self, email: &IdentityEmail) -> Result<Identity> {
    sqlx::query_as::<_, IdentityEntity>("select id, login, email from identity where email = $1")
      .bind(email.value())
      .fetch_optional(&self.connection)
      .await
      .map_err(|err| IdentityError::Repository(err.to_string()))
      .and_then(|opt| {
        opt.ok_or(IdentityError::IdentityByEmailNotFound(
          email.value().to_owned(),
        ))
      })
      .and_then(|ie| Identity::try_from(ie))
  }

  async fn create(&self, identity_for_create: &IdentityForCreate) -> Result<IdentityId> {
    let id = Uuid::new_v4();
    sqlx::query(
      "insert into identity (id, login, email, salt, password) values ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(identity_for_create.login.value())
    .bind(identity_for_create.email.value())
    .bind(identity_for_create.salt.value())
    .bind(identity_for_create.password.value())
    .execute(&self.connection)
    .await
    .map_err(|err| IdentityError::Repository(err.to_string()))?;

    Ok(IdentityId::from(id))
  }
}
