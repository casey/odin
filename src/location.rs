use crate::common::*;

#[derive(Debug)]
pub(crate) enum Location {
  Name { name: String },
  Path { path: PathBuf },
}

impl Location {
  pub(crate) fn name(name: impl ToString) -> Location {
    Location::Name {
      name: name.to_string(),
    }
  }

  pub(crate) fn path(path: impl Into<PathBuf>) -> Location {
    Location::Path { path: path.into() }
  }
}

impl<P: AsRef<Path>> From<P> for Location {
  fn from(path: P) -> Location {
    Location::path(path.as_ref())
  }
}

display_enum! {
  Location {
    Name { name } => {"\"{name}\"" }
    Path { path } => { "at `{path}`", path=path.display() }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display_name() {
    assert_eq!(Location::name("foo").to_string(), "\"foo\"");
  }

  #[test]
  fn display_path() {
    assert_eq!(Location::path("foo").to_string(), "at `foo`");
  }
}
