// stdlib
pub(crate) use std::{
  collections::{BTreeMap, HashMap},
  env,
  fmt::{self, Display, Formatter},
  fs, io,
  iter::FromIterator,
  path::{Path, PathBuf},
  process::{self, Command, ExitStatus},
  str::{self, FromStr},
};

// dependencies
pub(crate) use itertools::Itertools;
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde_derive::{Deserialize, Serialize};
pub(crate) use structopt::{clap::AppSettings, StructOpt};
pub(crate) use tera::{Tera, Value};
pub(crate) use url::Url;
pub(crate) use xdg::{BaseDirectories, BaseDirectoriesError};

// constants
pub(crate) use crate::default_config::DEFAULT_CONFIG;

// structs and enums
pub(crate) use crate::{
  config::Config, context::Context, error::Error, fn_cmd::FnCmd, fn_env::FnEnv, ft_join::FtJoin,
  location::Location, opt::Opt, query::Query, subcommand::Subcommand,
  template_parse_error::TemplateParseError, use_color::UseColor,
};

// traits
pub(crate) use crate::{filter::Filter, function::Function};

// testing
#[cfg(test)]
pub(crate) use crate::testing::{self, common::*};
