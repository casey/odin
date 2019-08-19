use crate::common::*;

#[derive(Debug)]
pub(crate) struct TemplateParseError {
  pub(crate) text: String,
  pub(crate) tera_error: tera::Error,
}

impl Display for TemplateParseError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.tera_error, self.text)
  }
}
