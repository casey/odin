use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  AliasTargetUnknown {
    name: String,
    target: String,
  },
  BaseDirectories {
    base_directories_error: BaseDirectoriesError,
  },
  BrowserExitStatus {
    url: Url,
    exit_status: ExitStatus,
    stdout: String,
    stderr: String,
  },
  BrowserLaunch {
    io_error: io::Error,
  },
  ConfigDeserialize {
    location: Location,
    yaml_error: serde_yaml::Error,
  },
  ConfigIo {
    location: Location,
    io_error: io::Error,
  },
  ConfigSerialize {
    yaml_error: serde_yaml::Error,
  },
  Internal {
    message: String,
  },
  TemplateParse {
    errors: Vec<TemplateParseError>,
  },
  TemplateRender {
    text: String,
    tera_error: tera::Error,
  },
  TemplateUnknown {
    name: String,
  },
  UrlParse {
    text: String,
    url_parse_error: url::ParseError,
  },
}

impl Error {
  pub(crate) fn internal(message: impl ToString) -> Error {
    Error::Internal {
      message: message.to_string(),
    }
  }
}

impl From<BaseDirectoriesError> for Error {
  fn from(base_directories_error: BaseDirectoriesError) -> Error {
    Error::BaseDirectories {
      base_directories_error,
    }
  }
}

display_enum! {
  Error {
    AliasTargetUnknown { name, target } => {
      "Alias `{name}` has unknown target `{target}`"
    }
    BaseDirectories { base_directories_error } => {
      "Failed to access user directory: {base_directories_error}"
    }
    BrowserExitStatus { url, exit_status, stdout, stderr } => {
      "Browser launched for `{}` exited with status: {}{}{}",
      url,
      exit_status,
      if !stdout.is_empty() {
        format!("\n\nstdout:\n{}\n", textwrap::indent(stdout, "  "))
      } else {
        "".to_string()
      },
      if !stderr.is_empty() {
        format!("\n\nstderr:\n{}\n", textwrap::indent(stderr, "  "))
      } else {
        "".to_string()
      },
    }
    BrowserLaunch { io_error } => {
      "Failed to launch browser: {io_error}",
    }
    ConfigDeserialize { location, yaml_error } => {
      "Failed to deserialize configuration {location}: {yaml_error}"
    }
    ConfigIo { location, io_error } => {
      "I/O error reading config file {location}: {io_error}"
    }
    ConfigSerialize { yaml_error } => {
      "Failed to serialize configuration: {yaml_error}"
    }
    Internal { message } => {
      "Internal error: {message}\n\
       This might be due to a bug in odin.\n\
       Consider filing an issue: https://github.com/casey/odin/issues/new",
    }
    TemplateParse { errors } if errors.len() == 1 => {
      "{}", errors[0]
    }
    TemplateParse { errors } => {
      "Errors parsing templates:\n{}", errors.iter().format("\n")
    }
    TemplateRender { text, tera_error } => {
      "{tera_error}: {text}"
    }
    TemplateUnknown { name } => {
      "Template `{name}` unknown"
    }
    UrlParse { text, url_parse_error } => {
      "Failed to parse rendered template output `{text}` as URL: {url_parse_error}"
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use Error::*;

  display_test! {
    name: display_alias_target_unknown,
    want: "Alias `foo` has unknown target `bar`",
    have: AliasTargetUnknown {
      name: "foo".to_string(),
      target: "bar".to_string(),
    },
  }

  display_test! {
    name: display_config_deserialize,
    want: "Failed to deserialize configuration at `foo`: EOF while parsing a value",
    have: ConfigDeserialize {
      location: PathBuf::from("foo").into(),
      yaml_error: serde_yaml::from_str::<Config>("").unwrap_err(),
    },
  }
}
