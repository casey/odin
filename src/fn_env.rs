use crate::common::*;

pub(crate) struct FnEnv;

#[derive(Deserialize)]
pub(crate) struct Arguments {
  var: String,
}

impl Function for FnEnv {
  type Arguments = Arguments;

  fn name(&self) -> &'static str {
    "env"
  }

  fn usage(&self) -> &'static str {
    r#"var=NAME""#
  }

  fn call(&self, args: Self::Arguments) -> Result<Value, String> {
    env::var(&args.var)
      .map(Value::String)
      .map_err(|var_error| match var_error {
        env::VarError::NotPresent => format!("env var `{}` not present", args.var),
        env::VarError::NotUnicode(os_string) => format!(
          "env var `{}` not unicode: {}",
          args.var,
          os_string.to_string_lossy()
        ),
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() -> Result<(), Error> {
    let context = testing::context(vec![(
      "example",
      r#"https://example.com/search?q={{args | join}}-{{env(var="FOO")}}"#,
    )])?;

    env::set_var("FOO", "BAR");

    let result = context.render("example", &["baz"])?;

    assert_eq!(result.as_str(), "https://example.com/search?q=baz-BAR");

    Ok(())
  }

  #[test]
  fn missing() -> Result<(), Error> {
    let context = testing::context(vec![(
      "example",
      r#"https://example.com/search?q={{args | join}}-{{env(var="BAZZZZZZZZZ")}}"#,
    )])?;

    assert_eq!(
      testing::render_error_message(&context, "example", "baz"),
      "env var `BAZZZZZZZZZ` not present",
    );

    Ok(())
  }

  #[test]
  fn not_unicode() -> Result<(), Error> {
    let context = testing::context(vec![(
      "example",
      r#"https://example.com/search?q={{args | join}}-{{env(var="BAR")}}"#,
    )])?;

    let non_unicode = testing::non_unicode_os_string();

    env::set_var("BAR", non_unicode);

    assert_eq!(
      testing::render_error_message(&context, "example", "baz"),
      "env var `BAR` not unicode: �"
    );

    Ok(())
  }
}
