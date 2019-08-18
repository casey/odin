use crate::common::*;

#[derive(Deserialize, Default, Eq, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub(crate) struct Config {
  pub(crate) root: bool,
  #[serde(skip)]
  pub(crate) parent: Option<Box<Config>>,
  pub(crate) templates: BTreeMap<String, String>,
  pub(crate) aliases: BTreeMap<String, String>,
}

impl Config {
  pub(crate) const FILE_NAME: &'static str = "odin.yaml";

  pub(crate) fn load(override_path: Option<PathBuf>) -> Result<Config, Error> {
    if let Some(override_path) = override_path {
      Self::from_path(&override_path)
    } else if let Some(config_path) = BaseDirectories::new()?.find_config_file(Self::FILE_NAME) {
      Self::from_path(&config_path)
    } else {
      Self::from_yaml(DEFAULT_CONFIG, Location::name("default config"))
    }
  }

  pub(crate) fn from_yaml(yaml: &str, location: impl Into<Location>) -> Result<Config, Error> {
    let config = serde_yaml::from_str(yaml).map_err(|yaml_error| Error::ConfigDeserialize {
      yaml_error,
      location: location.into(),
    })?;

    Self::extend(config)
  }

  pub(crate) fn from_path(path: &Path) -> Result<Config, Error> {
    let yaml = fs::read_to_string(&path).map_err(|io_error| Error::ConfigIo {
      path: path.to_owned(),
      io_error,
    })?;

    Self::from_yaml(&yaml, path)
  }

  fn extend(mut config: Config) -> Result<Config, Error> {
    if !config.root {
      let parent = Config::from_yaml(DEFAULT_CONFIG, Location::name("default config"))?;
      config.parent = Some(Box::new(parent));
    }

    config.check()?;

    Ok(config)
  }

  fn check(&self) -> Result<(), Error> {
    for (name, target) in &self.aliases {
      self.check_alias(name, target)?;
    }

    Ok(())
  }

  fn check_alias(&self, name: &str, target: &str) -> Result<(), Error> {
    if self.templates.contains_key(target) {
      Ok(())
    } else if let Some(parent) = &self.parent {
      parent.check_alias(name, target)
    } else {
      Err(Error::AliasTargetUnknown {
        name: name.to_string(),
        target: target.to_string(),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn root_has_no_parent() -> Result<(), Error> {
    let config = testing::config(
      "
      root: true
    ",
    )?;

    assert!(config.root);
    assert!(config.parent.is_none());
    assert!(config.templates.is_empty());
    assert!(config.aliases.is_empty());

    Ok(())
  }

  #[test]
  fn child_has_parent() -> Result<(), Error> {
    let config = testing::config(
      "
      root: false
    ",
    )?;

    assert!(!config.root);
    assert!(config.parent.is_some());
    assert!(config.templates.is_empty());
    assert!(config.aliases.is_empty());

    Ok(())
  }

  #[test]
  fn child_inherits_from_default() -> Result<(), Error> {
    let config = testing::config(
      "
      root: false
    ",
    )?;

    let parent = config.parent.unwrap();

    let default = testing::config(DEFAULT_CONFIG)?;

    assert_eq!(parent.as_ref(), &default);

    Ok(())
  }

  #[test]
  fn unknown_alias() -> Result<(), Error> {
    let result = testing::config(
      "
      aliases:
        foo: bar
    ",
    );

    match result {
      Err(Error::AliasTargetUnknown { name, target }) => {
        assert_eq!(name, "foo");
        assert_eq!(target, "bar");
      }
      Err(err) => panic!("unexpected error: {:?}", err),
      Ok(ok) => panic!("expected error but got: {:?}", ok),
    }

    Ok(())
  }

  #[test]
  fn alias_in_parent_does_not_find_target_in_child() -> Result<(), Error> {
    let result = testing::config(
      "
      aliases:
        foo: bar
    ",
    );

    match result {
      Err(Error::AliasTargetUnknown { name, target }) => {
        assert_eq!(name, "foo");
        assert_eq!(target, "bar");
      }
      Err(err) => panic!("unexpected error: {:?}", err),
      Ok(ok) => panic!("expected error but got: {:?}", ok),
    }

    Ok(())
  }

}
