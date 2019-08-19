use crate::common::*;

use ansi_term::{Color, Style};
use atty::Stream;
use libc::EXIT_FAILURE;

#[derive(StructOpt)]
#[structopt(raw(setting = "AppSettings::ColoredHelp"))]
#[structopt(help_message = "Print help information")]
#[structopt(version_message = "Print version information")]
pub(crate) struct Opt {
  #[structopt(name = "CONFIG", long = "--config")]
  config_path: Option<PathBuf>,
  #[structopt(
    name = "COLOR",
    long = "--color",
    default_value = "auto",
    raw(possible_values = "UseColor::variants()")
  )]
  use_color: UseColor,
  #[structopt(subcommand)]
  subcommand: Subcommand,
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), i32> {
    let color = match self.use_color {
      UseColor::Auto => atty::is(Stream::Stderr),
      UseColor::Always => true,
      UseColor::Never => false,
    };

    if let Err(err) = self.inner() {
      let error = if color {
        Style::new().fg(Color::Red).bold()
      } else {
        Style::new()
      };

      let message = if color {
        Style::new().bold()
      } else {
        Style::new()
      };

      eprintln!(
        "{} {}",
        error.paint("error:"),
        message.paint(err.to_string())
      );

      Err(EXIT_FAILURE)
    } else {
      Ok(())
    }
  }

  fn inner(self) -> Result<(), Error> {
    let config = Config::load(self.config_path)?;
    self.subcommand.run(&config)
  }
}
