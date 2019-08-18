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

#[cfg(test)]
mod testing;

use crate::common::*;

fn main() -> Result<(), Error> {
  Opt::from_args().run()
}
