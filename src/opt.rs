use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Opt {
  pub(crate) template: String,
  pub(crate) query: String,
}
