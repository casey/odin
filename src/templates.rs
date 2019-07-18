use crate::common::*;

pub(crate) struct Templates {
  tera: Tera,
}

impl Templates {
  pub(crate) fn from(templates: &BTreeMap<String, String>) -> Result<Templates, Error> {
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

    Ok(Templates { tera })
  }

  pub(crate) fn render(&self, name: &str, context: &Context) -> Result<Url, Error> {
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
