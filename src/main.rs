mod common;
mod config;
mod error;
mod opt;
mod template_parse_error;
mod templates;

use crate::common::*;

fn main() -> Result<(), Error> {
  let opt = Opt::from_args();

  let base_directories = BaseDirectories::new()?;

  let config_path = base_directories
    .find_config_file("odin.yaml")
    .ok_or(Error::ConfigMissing)?;

  let config_reader = File::open(&config_path).map_err(|io_error| Error::ConfigIo {
    path: config_path.clone(),
    io_error,
  })?;

  let config: Config =
    serde_yaml::from_reader(config_reader).map_err(|yaml_error| Error::ConfigDeserialize {
      path: config_path.clone(),
      yaml_error,
    })?;

  let templates = Templates::from(&config.templates)?;

  let mut context = Context::new();
  context.insert("query", &opt.query);

  let url = templates.render(&opt.template, &context)?;

  let output = webbrowser::open(&url.as_str()).map_err(|io_error| Error::BrowserOpen {
    url: url.clone(),
    io_error,
  })?;

  if !output.status.success() {
    return Err(Error::BrowserExitStatus {
      url: url.clone(),
      exit_status: output.status,
    });
  }

  Ok(())
}
