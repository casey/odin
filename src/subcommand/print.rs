use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Print {
  #[structopt(flatten)]
  query: Query,
}

impl Print {
  pub(crate) fn run(self, config: &Config) -> Result<(), Error> {
    let url = self.query.render(config)?;

    println!("{}", url);

    Ok(())
  }
}
