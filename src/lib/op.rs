pub enum Reason {
  /// Skip link because there is no `from` file found
  NoSource,

  /// Skip link if there is a `to` file that you want to skip
  DestExists,
}

impl std::convert::From<&Reason> for String {
  fn from(val: &Reason) -> String {
    match val {
      Reason::DestExists => String::from("Destination file already exists"),
      Reason::NoSource => String::from("Source file does not exists"),
    }
  }
}

pub enum Op {
  /// Create link if there is no such file
  Create,

  /// Replace link if there is a file
  Replace,

  Skip(Reason),
}

impl std::convert::Into<String> for Op {
  fn into(self) -> String {
    match &self {
      Op::Create => String::from("Create Link"),
      Op::Replace => String::from("Replace Link"),
      Op::Skip(reason) => format!("Skip({})", String::from(reason)),
    }
  }
}
