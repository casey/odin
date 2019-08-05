use crate::common::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
  pub(crate) aliases: Option<BTreeMap<String, String>>,
  pub(crate) templates: BTreeMap<String, String>,
}

impl Config {
  pub(crate) const FILE_NAME: &'static str = "odin.yaml";

  /// Instantiate config from YAML file at `path`
  pub(crate) fn load(path: &Path) -> Result<Config, Error> {
    let file = File::open(&path).map_err(|io_error| Error::ConfigIo {
      path: path.to_owned(),
      io_error,
    })?;

    Self::from_reader(file).map_err(|yaml_error| Error::ConfigDeserialize {
      path: path.to_owned(),
      yaml_error,
    })
  }

  /// Instantiate config from reader
  pub(crate) fn from_reader(reader: impl Read) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_reader(reader)
  }

  /// Look up `name` in aliases and templates
  pub(crate) fn resolve<'a>(&'a self, name: &'a str) -> Result<&'a str, Error> {
    let template: &str = self
      .aliases
      .as_ref()
      .map(|aliases| match aliases.get(name) {
        Some(target) => target,
        None => name,
      })
      .unwrap_or(name);

    if !self.templates.contains_key(template) {
      Err(Error::TemplateUnknown {
        name: name.to_owned(),
      })
    } else {
      Ok(template)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn resolve() -> Result<(), Error> {
    let config = testing::config(
      "
      templates:
        foo: FOO

      aliases:
        bar: foo
    ",
    )?;

    assert_eq!(config.resolve("foo")?, "foo");
    assert_eq!(config.resolve("bar")?, "foo");

    Ok(())
  }
}
