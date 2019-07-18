use crate::common::*;

use tera::{Map, Value};

pub(crate) trait Function: Send + Sync + Sized + 'static {
  type Arguments: DeserializeOwned;

  fn name(&self) -> &'static str;

  fn usage(&self) -> &'static str;

  fn call(&self, args: Self::Arguments) -> Result<Value, String>;

  fn register(self, tera: &mut Tera) {
    tera.register_function(self.name(), self.boxed())
  }

  fn boxed(self) -> Box<Fn(&HashMap<String, Value>) -> Result<Value, tera::Error> + Send + Sync> {
    Box::new(move |args| self.dispatch(args))
  }

  fn dispatch(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

    self.call(args).map_err(|msg| tera::Error::msg(msg))
  }
}
