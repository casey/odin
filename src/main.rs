mod common;
mod config;
mod context;
mod error;
mod fn_cmd;
mod fn_env;
mod function;
mod opt;
mod template_parse_error;

#[cfg(test)]
mod testing;

use crate::common::*;

fn main() -> Result<(), Error> {
  let opt = Opt::from_args();

  let config_path = opt.config_path()?;

  let config = Config::load(&config_path)?;

  let templates = Context::new(&config.templates)?;

  let url = templates.render(&opt.template, &opt.query)?;

  if opt.print {
    println!("{}", url.as_str());
    return Ok(());
  }

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
