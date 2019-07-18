use crate::common::*;

pub(crate) struct Context {
  tera: Tera,
}

impl Context {
  pub(crate) fn new(templates: &BTreeMap<String, String>) -> Result<Context, Error> {
    let mut tera = Tera::default();

    let errors = templates
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

    Ok(Context { tera })
  }

  pub(crate) fn render(&self, name: &str, query: &str) -> Result<Url, Error> {
    let mut context = tera::Context::new();
    context.insert("query", query);

    let text = self
      .tera
      .render(name, context)
      .map_err(|tera_error| Error::TemplateRender {
        name: name.to_owned(),
        tera_error,
      })?;

    Url::parse(&text).map_err(|url_parse_error| Error::UrlParse {
      text,
      url_parse_error,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() -> Result<(), Error> {
    let context = testing::context(vec![("example", "https://example.com/search?q={{query}}")])?;

    let result = context.render("example", "XYZ")?;

    assert_eq!(result.as_str(), "https://example.com/search?q=XYZ");

    Ok(())
  }
}
