use crate::common::*;

use tera::Value;

pub(crate) struct FnCmd;

#[derive(Deserialize)]
pub(crate) struct Arguments {
  bin: String,
  args: Vec<String>,
}

impl Function for FnCmd {
  type Arguments = Arguments;

  fn name(&self) -> &'static str {
    "cmd"
  }

  fn usage(&self) -> &'static str {
    r#"bin=NAME, arg=[ARG1, ARG2 ...]"#
  }

  fn call(&self, args: Self::Arguments) -> Result<Value, String> {
    let cmd = || {
      let mut cmd = args.bin.clone();

      for arg in &args.args {
        cmd.push(' ');
        cmd.push_str(arg);
      }

      cmd
    };

    let output = Command::new(&args.bin)
      .args(&args.args)
      .output()
      .map_err(|io_error| format!("I/O error running `{}`: {}", cmd(), io_error))?;

    if !output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
      let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

      return Err(format!(
        "command `{}` returned error status: {}\nstdout: {}\nstderr: {}",
        cmd(),
        output.status,
        stdout,
        stderr,
      ));
    }

    str::from_utf8(&output.stdout)
      .map(|text| Value::String(text.trim().to_owned()))
      .map_err(|utf8_error| format!("command `{}` stdout not valid utf8: {}", cmd(), utf8_error))
  }
}
