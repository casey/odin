use executable_path::executable_path;
use std::{io, process::Command};

macro_rules! case {
  {
    name:     $name:ident,
    template: $template:expr,
    query:    $query:expr,
    rendered: $rendered:expr,
  } => {
    #[test]
    fn $name() -> io::Result<()> {
      let template = $template;
      let query = $query;
      let rendered = $rendered;

      let have = print(template, query)?;

      assert_eq!(have, rendered);

      Ok(())
    }
  }
}

fn print(template: &str, query: &str) -> io::Result<String> {
  let executable = executable_path("odin");

  let output = Command::new(executable)
    // .arg("--config")
    // .arg("odin.yaml")
    .arg("print")
    .arg(template)
    .arg(query)
    .output()?;

  Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[allow(unused)]
fn cmd(bin: &str, args: &[&str]) -> String {
  let output = Command::new(bin).args(args).output().unwrap();

  String::from_utf8_lossy(&output.stdout).into_owned()
}

case! {
  name:     google,
  template: "google",
  query:    "how is babby formed",
  rendered: "https://www.google.com/search?q=how%20is%20babby%20formed\n",
}

case! {
  name:     stack_overflow,
  template: "stack-overflow",
  query:    "why program crash",
  rendered: "https://stackoverflow.com/search?q=why%20program%20crash\n",
}

case! {
  name:     github,
  template: "github",
  query:    "just",
  rendered: "https://github.com/search?q=just\n",
}
