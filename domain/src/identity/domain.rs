use uuid::Uuid;

use super::Result;
use crate::crypt::sign_into_b64_url;

// region: -- IdentityId
pub struct IdentityId(Uuid);

impl From<Uuid> for IdentityId {
  fn from(value: Uuid) -> Self {
    IdentityId(value)
  }
}

impl IdentityId {
  pub fn value(&self) -> &Uuid {
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
  pub fn value(&self) -> &String {
    &self.0
  }
}
// endregion: -- IdentityLogin

// region: -- IdentityEmail
pub struct IdentityEmail(String);

impl IdentityEmail {
  pub fn value(&self) -> &String {
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

// region: -- IdentityDecryptedPassword
pub struct IdentityDecryptedPassword(String);

impl IdentityDecryptedPassword {
  pub fn value(&self) -> &String {
    &self.0
  }
}

impl TryFrom<String> for IdentityDecryptedPassword {
  type Error = super::Error;

  fn try_from(value: String) -> super::Result<Self> {
    if value.is_empty() {
      Err(super::Error::IdentityPasswordIsEmpty)
    } else {
      Ok(Self(value))
    }
  }
}
// endregion: -- IdentityDecryptedPassword

// region: -- IdentityPasswordSalt
pub struct IdentityPasswordSalt(Uuid);

impl IdentityPasswordSalt {
  pub fn create() -> Self {
    Self(Uuid::new_v4())
  }

  pub fn value(&self) -> &Uuid {
    &self.0
  }
}
// endregion: -- IdentityPasswordSalt

// region: -- IdentityEncryptedPassword
pub struct IdentityEncryptedPassword(String);

impl IdentityEncryptedPassword {
  pub fn create(
    decrypted_password: &IdentityDecryptedPassword,
    password_salt: &IdentityPasswordSalt,
    passwod_key: &str,
  ) -> Result<Self> {
    let encrypted_password = sign_into_b64_url(
      passwod_key.as_bytes(),
      &password_salt.value().to_string(),
      decrypted_password.value(),
    )?;

    Ok(IdentityEncryptedPassword(encrypted_password))
  }

  pub fn value(&self) -> &String {
    &self.0
  }
}
// endregion: -- IdentityEncryptedPassword

pub struct IdentityForCreate {
  pub login: IdentityLogin,
  pub email: IdentityEmail,
  pub salt: IdentityPasswordSalt,
  pub password: IdentityEncryptedPassword,
}

pub struct Identity {
  pub id: IdentityId,
  pub login: IdentityLogin,
  pub email: IdentityEmail,
}
