use crate::common::*;

pub(crate) struct Context {
  parent: Option<Box<Context>>,
  aliases: BTreeMap<String, String>,
  templates: BTreeMap<String, String>,
  tera: Tera,
}

impl Context {
  pub(crate) fn new(config: &Config) -> Result<Context, Error> {
    let parent = if let Some(parent) = config.parent.as_ref() {
      Some(Box::new(Context::new(parent)?))
    } else {
      None
    };

    Ok(Context {
      parent,
      templates: config.templates.clone(),
      aliases: config.aliases.clone(),
      tera: Self::tera(config)?,
    })
  }

  fn tera(config: &Config) -> Result<Tera, Error> {
    let mut tera = Tera::default();

    let errors = config
      .templates
      .iter()
      .flat_map(|(name, text)| {
        tera
          .add_raw_template(name, text)
          .map_err(|tera_error| TemplateParseError {
            name: name.clone(),
            text: text.clone(),
            tera_error,
          })
          .err()
      })
      .collect::<Vec<TemplateParseError>>();

    if !errors.is_empty() {
      return Err(Error::TemplateParse { errors });
    }

    FnCmd.register(&mut tera);
    FnEnv.register(&mut tera);
    FtJoin.register(&mut tera);

    Ok(tera)
  }

  pub(crate) fn render(&self, name: &str, args: &[&str]) -> Result<Url, Error> {
    let target = self
      .aliases
      .get(name)
      .map(|target| target.as_str())
      .unwrap_or(name);

    if self.templates.contains_key(target) {
      let mut context = tera::Context::new();
      let args = Value::Array(
        args
          .iter()
          .map(|arg| Value::String(arg.to_string()))
          .collect(),
      );
      context.insert("args", &args);

      let text = self
        .tera
        .render(target, context)
        .map_err(|tera_error| Error::TemplateRender {
          name: name.to_owned(),
          tera_error,
        })?;

      Url::parse(&text).map_err(|url_parse_error| Error::UrlParse {
        text,
        url_parse_error,
      })
    } else if let Some(parent) = &self.parent {
      parent.render(target, args)
    } else {
      Err(Error::TemplateUnknown {
        name: name.to_owned(),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() -> Result<(), Error> {
    let context = testing::context(vec![(
      "example",
      "https://example.com/search?q={{args | join}}",
    )])?;

    let result = context.render("example", &["XYZ"])?;

    assert_eq!(result.as_str(), "https://example.com/search?q=XYZ");

    Ok(())
  }

  #[test]
  fn child_template_shadows_parent() -> Result<(), Error> {
    let config = testing::config(
      "
      templates:
        crates-io: https://foo.rs/{{args | join}}
    ",
    )?;

    let query = testing::query("crates-io", &["bar", "baz"]);

    let url = query.render(&config)?;

    assert_eq!(url.to_string(), "https://foo.rs/bar%20baz");

    Ok(())
  }

  #[test]
  fn child_alias_finds_child_target() -> Result<(), Error> {
    let config = testing::config(
      "
      aliases:
        ci: crates-io

      templates:
        crates-io: https://foo.rs/{{args | join}}
    ",
    )?;

    let query = testing::query("ci", &["bar", "baz"]);

    let url = query.render(&config)?;

    assert_eq!(url.to_string(), "https://foo.rs/bar%20baz");

    Ok(())
  }

  #[test]
  fn parent_template_resolves() -> Result<(), Error> {
    let config = testing::config(
      "
      root: false
    ",
    )?;

    let query = testing::query("crates-io", &["bar", "baz"]);

    let url = query.render(&config)?;

    assert_eq!(url.to_string(), "https://crates.io/search?q=bar%20baz");

    Ok(())
  }

  #[test]
  fn child_alias_resolves_to_parent_target() -> Result<(), Error> {
    let config = testing::config(
      "
      aliases:
        foo: crates-io
    ",
    )?;

    let query = testing::query("foo", &["bar", "baz"]);

    let url = query.render(&config)?;

    assert_eq!(url.to_string(), "https://crates.io/search?q=bar%20baz");
    Ok(())
  }

  #[test]
  fn alias_in_parent_ignores_targets_in_child() -> Result<(), Error> {
    let config = testing::config(
      "
      templates:
        crates-io: https://foo.rs/{{args | join}}
    ",
    )?;

    let query = testing::query("ci", &["bar", "baz"]);

    let url = query.render(&config)?;

    assert_eq!(url.to_string(), "https://crates.io/search?q=bar%20baz");

    Ok(())
  }

}
