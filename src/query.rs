use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Query {
  #[structopt(name = "TEMPLATE")]
  pub(crate) name: String,
  #[structopt(name = "QUERY")]
  pub(crate) args: Vec<String>,
}

impl Query {
  pub(crate) fn render(&self, config: &Config) -> Result<Url, Error> {
    let context = Context::new(&config)?;

    let args = self.args.iter().map(AsRef::as_ref).collect::<Vec<&str>>();

    let url = context.render(&self.name, &args)?;

    Ok(url)
  }
}
