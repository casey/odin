use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
  #[structopt(name = "TEMPLATE")]
  pub(crate) template: String,
  #[structopt(name = "QUERY")]
  pub(crate) query: String,
  #[structopt(name = "CONFIG", long = "--config")]
  pub(crate) config_path: Option<PathBuf>,
  #[structopt(name = "PRINT", long = "print")]
  pub(crate) print: bool,
}

impl Opt {
  pub(crate) fn config_path(&self) -> Result<PathBuf, Error> {
    if let Some(config_path) = &self.config_path {
      return Ok(config_path.clone());
    }

    let base_directories = BaseDirectories::new()?;

    base_directories
      .find_config_file(Config::FILE_NAME)
      .ok_or(Error::ConfigMissing)
  }
}
