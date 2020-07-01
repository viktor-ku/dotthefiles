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
      os_info::Type::Linux
      | os_info::Type::Redhat
      | os_info::Type::RedHatEnterprise
      | os_info::Type::Ubuntu
      | os_info::Type::Pop
      | os_info::Type::Debian
      | os_info::Type::Arch
      | os_info::Type::Centos
      | os_info::Type::Fedora
      | os_info::Type::SUSE
      | os_info::Type::openSUSE
      | os_info::Type::OracleLinux
      | os_info::Type::Solus
      | os_info::Type::Manjaro
      | os_info::Type::Alpine => Self::Linux,
      os_info::Type::Macos => Self::Darwin,
      os_info::Type::Windows => Self::Win,
      _ => Self::Unknown,
    }
  }
}
