use crate::common::*;

#[derive(Deserialize)]
pub(crate) struct Config {
  pub(crate) templates: BTreeMap<String, String>,
}
