use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
  #[structopt(name = "CONFIG", long = "--config")]
  config_path: Option<PathBuf>,
  #[structopt(subcommand)]
  subcommand: Subcommand,
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    let config = Config::load(self.config_path)?;

    self.subcommand.run(&config)
  }
}
