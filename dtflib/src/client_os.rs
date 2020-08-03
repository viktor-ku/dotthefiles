#[derive(Debug, PartialEq)]
pub enum Type {
  /// Alpine Linux (<https://en.wikipedia.org/wiki/Alpine_Linux>).
  Alpine,
  /// Amazon Linux AMI (<https://en.wikipedia.org/wiki/Amazon_Machine_Image#Amazon_Linux_AMI>).
  Amazon,
  /// Arch Linux (<https://en.wikipedia.org/wiki/Arch_Linux>).
  Arch,
  /// CentOS (<https://en.wikipedia.org/wiki/CentOS>).
  Centos,
  /// Debian (<https://en.wikipedia.org/wiki/Debian>).
  Debian,
  /// Fedora (<https://en.wikipedia.org/wiki/Fedora_(operating_system)>).
  Fedora,
  /// Linux based operating system (<https://en.wikipedia.org/wiki/Linux>).
  Linux,
  /// Mac OS X/OS X/macOS (<https://en.wikipedia.org/wiki/MacOS>).
  Macos,
  /// Manjaro (<https://en.wikipedia.org/wiki/Manjaro>).
  Manjaro,
  /// openSUSE (<https://en.wikipedia.org/wiki/OpenSUSE>).
  OpenSUSE,
  /// Oracle Linux (<https://en.wikipedia.org/wiki/Oracle_Linux>).
  OracleLinux,
  /// Pop!_OS (<https://en.wikipedia.org/wiki/Pop!_OS>)
  Pop,
  /// Red Hat Linux (<https://en.wikipedia.org/wiki/Red_Hat_Linux>).
  Redhat,
  /// Red Hat Enterprise Linux (<https://en.wikipedia.org/wiki/Red_Hat_Enterprise_Linux>).
  RedHatEnterprise,
  /// Redox (<https://en.wikipedia.org/wiki/Redox_(operating_system)>).
  Redox,
  /// Solus (<https://en.wikipedia.org/wiki/Solus_(operating_system)>).
  Solus,
  /// SUSE Linux Enterprise Server (<https://en.wikipedia.org/wiki/SUSE_Linux_Enterprise>).
  SUSE,
  /// Ubuntu (<https://en.wikipedia.org/wiki/Ubuntu_(operating_system)>).
  Ubuntu,
  /// Windows (<https://en.wikipedia.org/wiki/Microsoft_Windows>).
  Windows,
  /// Unknown operating system.
  Unknown,
}

impl std::convert::From<os_info::Type> for Type {
  fn from(os_info_type: os_info::Type) -> Self {
    match &os_info_type {
      os_info::Type::Alpine => Self::Alpine,
      os_info::Type::Amazon => Self::Amazon,
      os_info::Type::Arch => Self::Arch,
      os_info::Type::Centos => Self::Centos,
      os_info::Type::Debian => Self::Debian,
      os_info::Type::Fedora => Self::Fedora,
      os_info::Type::Linux => Self::Linux,
      os_info::Type::Manjaro => Self::Manjaro,
      os_info::Type::openSUSE => Self::OpenSUSE,
      os_info::Type::OracleLinux => Self::OracleLinux,
      os_info::Type::Pop => Self::Pop,
      os_info::Type::Redhat => Self::Redhat,
      os_info::Type::RedHatEnterprise => Self::RedHatEnterprise,
      os_info::Type::Redox => Self::Redox,
      os_info::Type::Solus => Self::Solus,
      os_info::Type::SUSE => Self::SUSE,
      os_info::Type::Ubuntu => Self::Ubuntu,

      os_info::Type::Macos => Self::Macos,

      os_info::Type::Windows => Self::Windows,

      os_info::Type::Emscripten | os_info::Type::Android | os_info::Type::Unknown => Self::Unknown,

      _ => Self::Unknown,
    }
  }
}

impl std::convert::From<&str> for Type {
  fn from(val: &str) -> Self {
    match val {
      "alpine" => Type::Alpine,
      "amazon" => Type::Amazon,
      "arch" => Type::Arch,
      "centos" => Type::Centos,
      "debian" => Type::Debian,
      "fedora" => Type::Fedora,
      "linux" => Type::Linux,
      "macos" => Type::Macos,
      "manjaro" => Type::Manjaro,
      "opensuse" => Type::OpenSUSE,
      "oraclelinux" => Type::OracleLinux,
      "pop" => Type::Pop,
      "redhat" => Type::Redhat,
      "redhatenterprise" => Type::RedHatEnterprise,
      "redox" => Type::Redox,
      "solus" => Type::Solus,
      "suse" => Type::SUSE,
      "ubuntu" => Type::Ubuntu,
      "windows" => Type::Windows,
      _ => Type::Unknown,
    }
  }
}

impl Type {
  pub fn all<'a>() -> &'a [Self] {
    &[
      Type::Alpine,
      Type::Amazon,
      Type::Arch,
      Type::Centos,
      Type::Debian,
      Type::Fedora,
      Type::Linux,
      Type::Macos,
      Type::Manjaro,
      Type::OpenSUSE,
      Type::OracleLinux,
      Type::Pop,
      Type::Redhat,
      Type::RedHatEnterprise,
      Type::Redox,
      Type::Solus,
      Type::SUSE,
      Type::Ubuntu,
      Type::Windows,
      Type::Unknown,
    ]
  }
}

#[inline]
pub fn digest(input: Option<Type>) -> Type {
  input.unwrap_or_else(|| Type::from(os_info::get().os_type()))
}
