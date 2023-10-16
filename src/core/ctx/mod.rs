use uuid::Uuid;

#[derive(Clone)]
pub struct Ctx {
  request_id: Uuid,
}

impl Ctx {
  pub fn init() -> Self {
    Self {
      request_id: Uuid::new_v4(),
    }
  }

  pub fn request_id(&self) -> &Uuid {
    &self.request_id
  }
}
