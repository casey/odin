use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Open {
  #[structopt(flatten)]
  query: Query,
}

impl Open {
  pub(crate) fn run(self, config: &Config) -> Result<(), Error> {
    let url = self.query.render(config)?;

    let output = webbrowser::open(&url.as_str()).map_err(|io_error| Error::BrowserOpen {
      url: url.clone(),
      io_error,
    })?;

    if !output.status.success() {
      return Err(Error::BrowserExitStatus {
        url: url.clone(),
        exit_status: output.status,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
      });
    }

    Ok(())
  }
}
