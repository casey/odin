use crate::common::*;

#[derive(Deserialize)]
pub(crate) struct Config {
  pub(crate) templates: BTreeMap<String, String>,
}

impl Config {
  pub(crate) const FILE_NAME: &'static str = "odin.yaml";

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

  pub(crate) fn from_reader(reader: impl Read) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_reader(reader)
  }
}
