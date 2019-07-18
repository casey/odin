#[derive(Debug)]
pub(crate) struct TemplateParseError {
  pub(crate) name: String,
  pub(crate) text: String,
  pub(crate) tera_error: tera::Error,
}
