use crate::common::*;

pub(crate) mod common {
  pub(crate) use std::ffi::{OsStr, OsString};

  pub(crate) use std::error::Error as _;
}

pub(crate) fn context(
  templates: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
) -> Result<Context, Error> {
  let templates = templates
    .into_iter()
    .map(|(name, text)| (name.into(), text.into()))
    .collect::<BTreeMap<String, String>>();

  Context::new(&templates)
}

pub(crate) fn render_error_message(context: &Context, template: &str, query: &str) -> String {
  match context.render(template, query) {
    Err(Error::TemplateRender { name, tera_error }) => {
      assert_eq!(name, template);

      let source = tera_error
        .source()
        .unwrap()
        .downcast_ref::<tera::Error>()
        .unwrap();

      match &source.kind {
        tera::ErrorKind::Msg(msg) => msg.clone(),
        _ => panic!("unexpected tera error kind: {:?}", tera_error),
      }
    }
    Err(unexpected) => panic!("unexpeected error: {:?}", unexpected),
    Ok(_) => panic!("expected template render error"),
  }
}

#[cfg(unix)]
pub(crate) fn non_unicode_os_string() -> OsString {
  use std::os::unix::ffi::OsStrExt;

  const CONTINUATION_BYTE: u8 = 0b10000000;

  OsStr::from_bytes(&[CONTINUATION_BYTE]).to_owned()
}

#[cfg(windows)]
pub(crate) fn non_unicode_os_string() -> OsString {
  use std::os::windows::ffi::OsStringExt;

  const HIGH_SURROGATE: u16 = 0xD800;

  OsStr::from_wide(&[HIGH_SURROGATE, 0]).to_owned()
}
