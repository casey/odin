use crate::common::*;

use tera::{Map, Value};

type Boxed =
  Box<dyn Fn(&Value, &HashMap<String, Value>) -> Result<Value, tera::Error> + Send + Sync>;

pub(crate) trait Filter: Send + Sync + Sized + 'static {
  type Arguments: DeserializeOwned;
  type Input: DeserializeOwned;

  fn name(&self) -> &'static str;

  fn usage(&self) -> &'static str;

  fn call(&self, input: Self::Input, args: Self::Arguments) -> Result<Value, String>;

  fn register(self, tera: &mut Tera) {
    tera.register_filter(self.name(), self.boxed())
  }

  fn boxed(self) -> Boxed {
    Box::new(move |value, args| self.dispatch(value, args))
  }

  fn dispatch(&self, input: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let map = Map::from_iter(args.clone().into_iter());

    let args: Value = Value::Object(map);

    let args: Self::Arguments = serde_json::from_value(args).map_err(|serde_json_error| {
      format!(
        "usage: {}({}): {}",
        self.name(),
        self.usage(),
        serde_json_error
      )
    })?;

    let input: Self::Input = serde_json::from_value(input.clone()).map_err(|serde_json_error| {
      format!(
        "usage: {}({}): {}",
        self.name(),
        self.usage(),
        serde_json_error
      )
    })?;

    self.call(input, args).map_err(tera::Error::msg)
  }
}
