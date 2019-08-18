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
}

impl<P: AsRef<Path>> From<P> for Location {
  fn from(path: P) -> Location {
    Location::Path {
      path: path.as_ref().to_owned(),
    }
  }
}
