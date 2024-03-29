use crate::common::*;

pub(crate) mod dump;
pub(crate) mod open;
pub(crate) mod print;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  #[structopt(name = "open", about = "Open query with browser")]
  #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
  Open(open::Open),
  #[structopt(name = "print", about = "Print rendered query to stdout")]
  #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
  Print(print::Print),
  #[structopt(name = "dump", about = "Dump current config file to stdout")]
  #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
  Dump(dump::Dump),
}

impl Subcommand {
  pub(crate) fn run(self, config: &Config) -> Result<(), Error> {
    match self {
      Self::Print(print) => print.run(config),
      Self::Open(open) => open.run(config),
      Self::Dump(dump) => dump.run(config),
    }
  }
}
