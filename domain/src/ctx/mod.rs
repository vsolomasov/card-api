use uuid::Uuid;
use valuable::Valuable;

#[derive(Clone, Valuable)]
pub struct Ctx {
  request_id: String,
}

impl Ctx {
  pub fn init() -> Self {
    Self {
      request_id: Uuid::new_v4().to_string(),
    }
  }

  pub fn request_id(&self) -> &String {
    &self.request_id
  }
}
