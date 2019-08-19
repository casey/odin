macro_rules! display_enum {
  {
    $enum:ident {
      $(
        $variant:ident { $($field:ident),* $(,)? } $(if $condition:expr)? => {
          $fmt:expr $(, $($args:tt)*)?
        }
      )*
    }
  } => {
    impl Display for $enum {
      fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
          $(
            Self::$variant { $($field),* } $(if $condition)? =>
              display_enum_fmt!(f, $fmt, $($field),*, {$($($args)*)?}),
          )*
        }
      }
    }
  }
}

macro_rules! display_enum_fmt {
  {
    $formatter:expr, $fmt:expr, $($field:ident),*, {}
  } => {
    write!($formatter, $fmt, $($field=$field),*)
  };
  {
    $formatter:expr, $fmt:expr, $($field:ident),*, {$($args:tt)+}
  } => {
    write!($formatter, $fmt, $($args)+)
  }
}
