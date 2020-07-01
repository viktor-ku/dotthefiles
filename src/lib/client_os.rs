/// Handles clients OS type. Also, it is much more convenient to
/// map different kinds of the same OS (like linux might have different distros) to the
/// unified interface (in the future).
#[derive(Debug, PartialEq)]
pub enum Type {
  Linux,
  Darwin,
  Win,
  Unknown,
}

impl std::convert::From<&os_info::Type> for Type {
  fn from(os_info_type: &os_info::Type) -> Self {
    match os_info_type {
      os_info::Type::Linux => Self::Linux,
      os_info::Type::Macos => Self::Darwin,
      os_info::Type::Windows => Self::Win,
      _ => Self::Unknown,
    }
  }
}
