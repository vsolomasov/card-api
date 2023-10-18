use uuid::Uuid;

// region: -- IdentityId
pub struct IdentityId(Uuid);

impl From<Uuid> for IdentityId {
  fn from(value: Uuid) -> Self {
    IdentityId(value)
  }
}

impl IdentityId {
  pub fn raw(&self) -> &Uuid {
    &self.0
  }
}
// endregion: -- IdentityId

// region: -- IdentityLogin
pub struct IdentityLogin(String);

impl TryFrom<String> for IdentityLogin {
  type Error = super::Error;

  fn try_from(value: String) -> super::Result<Self> {
    if value.is_empty() {
      Err(super::Error::IdentityLoginIsEmpty)
    } else {
      Ok(Self(value))
    }
  }
}

impl IdentityLogin {
  pub fn raw(&self) -> &String {
    &self.0
  }
}
// endregion: -- IdentityLogin

// region: -- IdentityEmail
pub struct IdentityEmail(String);

impl IdentityEmail {
  pub fn raw(&self) -> &String {
    &self.0
  }
}

impl TryFrom<String> for IdentityEmail {
  type Error = super::Error;

  fn try_from(value: String) -> super::Result<Self> {
    if value.is_empty() {
      Err(super::Error::IdentityEmailIsEmpty)
    } else {
      Ok(Self(value))
    }
  }
}
// endregion: -- IdentityEmail

pub struct Identity {
  pub id: IdentityId,
  pub login: IdentityLogin,
  pub email: IdentityEmail,
}
