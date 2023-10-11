use super::SqlRepository;
use crate::core::identity::domain::{Identity, IdentityId, IdentityLogin};
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
}

impl TryFrom<IdentityEntity> for Identity {
  type Error = IdentityError;

  fn try_from(value: IdentityEntity) -> Result<Self> {
    let id = IdentityId::from(value.id);
    let login = IdentityLogin::try_from(value.login)?;
    Ok(Self { id, login })
  }
}

#[async_trait]
impl IdentityRepository for SqlRepository {
  async fn first_by_login(&self, login: IdentityLogin) -> Result<Identity> {
    sqlx::query_as::<_, IdentityEntity>("select id, login from identity where login = $1")
      .bind(login.raw())
      .fetch_optional(&self.connection)
      .await
      .map_err(|err| IdentityError::Repository(err.to_string()))
      .and_then(|opt| opt.ok_or(IdentityError::IdentityNotFound))
      .and_then(|ie| Identity::try_from(ie))
  }
}
