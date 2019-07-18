pub(crate) use std::{
  collections::{BTreeMap, HashMap},
  env,
  fs::File,
  io::{self, Read},
  iter::FromIterator,
  path::{Path, PathBuf},
  process::{Command, ExitStatus},
  str,
};

pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde_derive::Deserialize;
pub(crate) use structopt::StructOpt;
pub(crate) use tera::Tera;
pub(crate) use url::Url;
pub(crate) use xdg::{BaseDirectories, BaseDirectoriesError};

// structs and enums
pub(crate) use crate::{
  config::Config, context::Context, error::Error, fn_cmd::FnCmd, fn_env::FnEnv, opt::Opt,
  template_parse_error::TemplateParseError,
};

// traits
pub(crate) use crate::function::Function;

#[cfg(test)]
pub(crate) use crate::testing::{self, common::*};
