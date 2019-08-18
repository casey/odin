use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Dump {}

impl Dump {
  pub(crate) fn run(self, config: &Config) -> Result<(), Error> {
    println!("{}", Self::format(config)?);
    Ok(())
  }

  pub(crate) fn format(config: &Config) -> Result<String, Error> {
    #[derive(Serialize)]
    struct Pretty<'config> {
      templates: Option<&'config BTreeMap<String, String>>,
      #[serde(skip_serializing_if = "Option::is_none")]
      aliases: Option<&'config BTreeMap<String, String>>,
    }

    let pretty = Pretty {
      aliases: Some(&config.aliases).filter(|aliases| !aliases.is_empty()),
      templates: Some(&config.templates).filter(|templates| !templates.is_empty()),
    };

    let yaml =
      serde_yaml::to_string(&pretty).map_err(|yaml_error| Error::ConfigSerialize { yaml_error })?;

    let yaml = yaml.replace("\naliases:\n", "\n\naliases:\n");

    Ok(yaml.trim_start_matches("---\n").to_owned())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn templates() -> Result<(), Error> {
    let config = testing::config(
      "
      templates:
        foo: http://bar.com/{{query}}
    ",
    )?;

    assert_eq!(
      Dump::format(&config)?,
      "templates:\n  foo: \"http://bar.com/{{query}}\""
    );

    Ok(())
  }

  #[test]
  fn templates_and_aliases() -> Result<(), Error> {
    let config = testing::config(
      "
      aliases:
        bar: foo

      templates:
        foo: http://bar.com/{{query}}
    ",
    )?;

    assert_eq!(
      Dump::format(&config)?,
      "templates:\n  foo: \"http://bar.com/{{query}}\"\n\naliases:\n  bar: foo"
    );

    Ok(())
  }
}
