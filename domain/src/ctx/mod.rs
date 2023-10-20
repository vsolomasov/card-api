use uuid::Uuid;
use valuable::Valuable;

#[derive(Clone, Valuable)]
pub struct Ctx {
  request_id: String,
  uri: String,
  method: String,
  start_time: i128,
  request_time: Option<i128>,
}

impl Ctx {
  pub fn init(uri: String, method: String) -> Self {
    Self {
      request_id: Uuid::new_v4().to_string(),
      uri,
      method,
      start_time: time::OffsetDateTime::now_utc().unix_timestamp_nanos(),
      request_time: Option::None,
    }
  }

  pub fn request_id(&self) -> &String {
    &self.request_id
  }

  pub fn end_time(&mut self) {
    let end = time::OffsetDateTime::now_utc().unix_timestamp_nanos();
    let request_time = end - self.start_time;
    self.request_time = Option::Some(request_time / 1_000_000)
  }
}
