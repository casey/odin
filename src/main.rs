#[cfg(test)]
#[macro_use]
mod testing;

#[macro_use]
mod display_enum;

mod common;
mod config;
mod context;
mod default_config;
mod error;
mod filter;
mod fn_cmd;
mod fn_env;
mod ft_join;
mod function;
mod location;
mod opt;
mod query;
mod subcommand;
mod template_parse_error;
mod use_color;

use crate::common::*;

fn main() {
  if let Err(status_code) = Opt::from_args().run() {
    process::exit(status_code);
  }
}
