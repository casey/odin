use crate::common::*;

use tera::Value;

pub(crate) struct FtJoin;

#[derive(Deserialize)]
pub(crate) struct Arguments {
  sep: Option<String>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub(crate) struct Input {
  values: Vec<String>,
}

impl Filter for FtJoin {
  type Arguments = Arguments;
  type Input = Input;

  fn name(&self) -> &'static str {
    "join"
  }

  fn usage(&self) -> &'static str {
    r#"sep=SEPARATOR""#
  }

  fn call(&self, input: Self::Input, args: Self::Arguments) -> Result<Value, String> {
    let separator = args.sep.as_ref().map(|sep| sep.as_str()).unwrap_or(" ");

    Ok(Value::String(input.values.join(separator)))
  }
}
