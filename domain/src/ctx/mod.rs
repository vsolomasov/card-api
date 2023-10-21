use std::collections::BTreeMap;

use uuid::Uuid;

pub type Key = String;

#[derive(Clone)]
pub struct Ctx {
  id: Uuid,
  start_time: i128,
  fields: BTreeMap<String, String>,
}

impl Ctx {
  pub fn init(uri: String, method: String) -> Self {
    let id: Uuid = Uuid::new_v4();
    let start_time = time::OffsetDateTime::now_utc().unix_timestamp_nanos();

    let mut fields: BTreeMap<String, String> = BTreeMap::new();
    fields.insert("id".to_string(), id.to_string());
    fields.insert("uri".to_string(), uri);
    fields.insert("method".to_string(), method);

    Self {
      id,
      start_time,
      fields,
    }
  }

  pub fn id(&self) -> &Uuid {
    &self.id
  }

  pub fn add(&mut self, key: String, value: &dyn std::fmt::Debug) {
    self.fields.insert(key, format!("{:?}", value));
  }

  pub fn log(&self) -> &BTreeMap<String, String> {
    &self.fields
  }

  pub fn finish(&mut self) -> &BTreeMap<String, String> {
    let end = time::OffsetDateTime::now_utc().unix_timestamp_nanos();
    let request_time = (end - self.start_time) / 1_000_000;
    self
      .fields
      .insert("time".to_string(), request_time.to_string());
    &self.fields
  }
}
