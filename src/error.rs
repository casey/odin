use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  ConfigDeserialize {
    location: Location,
    yaml_error: serde_yaml::Error,
  },
  ConfigSerialize {
    yaml_error: serde_yaml::Error,
  },
  ConfigIo {
    path: PathBuf,
    io_error: io::Error,
  },
  AliasTargetUnknown {
    name: String,
    target: String,
  },
  TemplateUnknown {
    name: String,
  },
  TemplateParse {
    errors: Vec<TemplateParseError>,
  },
  TemplateRender {
    name: String,
    tera_error: tera::Error,
  },
  UrlParse {
    text: String,
    url_parse_error: url::ParseError,
  },
  BrowserExitStatus {
    url: Url,
    exit_status: ExitStatus,
    stdout: String,
    stderr: String,
  },
  BrowserOpen {
    url: Url,
    io_error: io::Error,
  },
  BaseDirectories {
    base_directories_error: BaseDirectoriesError,
  },
}

impl From<BaseDirectoriesError> for Error {
  fn from(base_directories_error: BaseDirectoriesError) -> Error {
    Error::BaseDirectories {
      base_directories_error,
    }
  }
}
