pub(crate) use std::{collections::BTreeMap, fs::File, io, path::PathBuf, process::ExitStatus};

pub(crate) use serde_derive::Deserialize;
pub(crate) use structopt::StructOpt;
pub(crate) use tera::{Context, Tera};
pub(crate) use url::Url;
pub(crate) use xdg::{BaseDirectories, BaseDirectoriesError};

pub(crate) use crate::{
  config::Config, error::Error, opt::Opt, template_parse_error::TemplateParseError,
  templates::Templates,
};
