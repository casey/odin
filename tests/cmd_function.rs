use std::{fs, io, process::Command};

use executable_path::executable_path;
use indoc::indoc;
use tempfile::TempDir;

macro_rules! case {
  {
    name:   $name:ident,
    config: $config:expr,
    stdout: $stdout:expr,
  } => {
    #[test]
    fn $name() -> io::Result<()> {
      let config = indoc!($config);

      let want = $stdout;

      let tempdir = TempDir::new()?;

      let config_path = tempdir.path().join("odin.yaml");

      fs::write(&config_path, config)?;

      let executable = executable_path("odin");

      let output = Command::new(executable)
        .arg("--config")
        .arg(config_path)
        .arg("print")
        .arg("foo")
        .arg("baz")
        .output()?;

      if !output.status.success() {
        panic!("odin invocation failed with status: {}", output.status);
      }

      let have = String::from_utf8_lossy(&output.stdout).into_owned();

      assert_eq!(have, want);

      Ok(())
    }
  }
}

case! {
  name:  echo,
  config: r#"
    templates:
      foo: https://{{cmd(bin="echo", args=["bar"])}}.{{args | join}}.com
  "#,
  stdout: "https://bar.baz.com/\n",
}
