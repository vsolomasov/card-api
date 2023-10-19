use std::collections::BTreeMap;

use serde_json::json;
use serde_json::to_string_pretty;
use serde_json::Value;
use tracing::field::Field;
use tracing::field::Visit;
use tracing::Subscriber;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub fn init() {
  tracing_subscriber::registry().with(AppLayer).init();
}

struct JsonVisitor<'a>(&'a mut BTreeMap<String, Value>);

impl<'a> Visit for JsonVisitor<'a> {
  fn record_f64(&mut self, field: &Field, value: f64) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
    self.0.insert(field.name().to_string(), json!(value));
  }

  fn record_error(
    &mut self,
    field: &tracing::field::Field,
    value: &(dyn std::error::Error + 'static),
  ) {
    self
      .0
      .insert(field.name().to_string(), json!(value.to_string()));
  }

  fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
    self
      .0
      .insert(field.name().to_string(), json!(format!("{:?}", value)));
  }
}

struct AppLayer;

impl<S> Layer<S> for AppLayer
where S: Subscriber
{
  fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
    let mut fields = BTreeMap::new();
    let mut visitor = JsonVisitor(&mut fields);
    event.record(&mut visitor);

    let output = json!({
      "target": event.metadata().target(),
      "level": event.metadata().level().as_str(),
      "fields": fields,
    });

    println!("{}", to_string_pretty(&output).unwrap());
  }
}
