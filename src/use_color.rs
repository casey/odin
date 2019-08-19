use crate::common::*;

const AUTO: &str = "auto";
const ALWAYS: &str = "always";
const NEVER: &str = "never";

#[derive(Copy, Clone)]
pub enum UseColor {
  Auto,
  Always,
  Never,
}

impl UseColor {
  pub fn variants() -> &'static [&'static str] {
    &[AUTO, ALWAYS, NEVER]
  }
}

impl FromStr for UseColor {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text {
      AUTO => Ok(Self::Auto),
      ALWAYS => Ok(Self::Always),
      NEVER => Ok(Self::Never),
      _ => Err(Error::internal(format!("UseColor::from_str: bad value: {}", text)).to_string()),
    }
  }
}
