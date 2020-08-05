use dtflib::client_os;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Ord, Serialize)]
pub enum Target {
  #[serde(alias = "linux")]
  Linux,

  #[serde(alias = "alpine")]
  Alpine,

  #[serde(alias = "amazon")]
  Amazon,

  #[serde(alias = "arch")]
  Arch,

  #[serde(alias = "centos")]
  Centos,

  #[serde(alias = "debian")]
  Debian,

  #[serde(alias = "fedora")]
  Fedora,

  #[serde(alias = "manjaro")]
  Manjaro,

  #[serde(alias = "opensuse")]
  OpenSUSE,

  #[serde(alias = "oraclelinux")]
  OracleLinux,

  #[serde(alias = "pop")]
  Pop,

  #[serde(alias = "redhat")]
  Redhat,

  #[serde(alias = "redhatenterprise", alias = "redhat enterprise")]
  RedHatEnterprise,

  #[serde(alias = "redox")]
  Redox,

  #[serde(alias = "solus")]
  Solus,

  #[serde(alias = "suse")]
  SUSE,

  #[serde(alias = "ubuntu")]
  Ubuntu,

  #[serde(alias = "darwin", alias = "macos", alias = "mac os")]
  Macos,

  #[serde(alias = "win", alias = "windows")]
  Windows,

  #[serde(
    alias = "*",
    alias = "any",
    alias = "all",
    alias = "every",
    alias = "each"
  )]
  Any,
}

impl Default for Target {
  fn default() -> Self {
    Self::Any
  }
}

impl Target {
  /// Weights are ordered like this:
  /// - `Any` (weights the most)
  /// - all linux distros
  /// - `Linux`, `Windows`, `Macos`
  ///
  /// `Linux` should weight less than its distros because if target list
  /// contains e.g. `Linux` and `Ubuntu`, then `Ubuntu` should be picked
  /// rather than Linux.
  ///
  /// Smallest -> More weight
  pub fn weight(&self) -> u8 {
    match self {
      Target::Any => 1,

      Target::Alpine
      | Target::Amazon
      | Target::Arch
      | Target::Centos
      | Target::Debian
      | Target::Fedora
      | Target::Manjaro
      | Target::OpenSUSE
      | Target::OracleLinux
      | Target::Pop
      | Target::Redhat
      | Target::RedHatEnterprise
      | Target::Redox
      | Target::Solus
      | Target::SUSE
      | Target::Ubuntu => 2,

      Target::Linux | Target::Macos | Target::Windows => 3,
    }
  }

  /// Returns the default directory name appropriate for the target
  pub fn dir(&self) -> &str {
    match self {
      Target::Macos => "macos",
      Target::Linux => "linux",
      Target::Windows => "windows",
      Target::Alpine => "alpine",
      Target::Amazon => "amazon",
      Target::Arch => "arch",
      Target::Centos => "centos",
      Target::Debian => "debian",
      Target::Fedora => "fedora",
      Target::Manjaro => "manjaro",
      Target::OpenSUSE => "opensuse",
      Target::OracleLinux => "oraclelinux",
      Target::Pop => "pop",
      Target::Redhat => "redhat",
      Target::RedHatEnterprise => "redhatenterprise",
      Target::Redox => "redox",
      Target::Solus => "solus",
      Target::SUSE => "suse",
      Target::Ubuntu => "ubuntu",
      Target::Any => "",
    }
  }

  /// Given that we know the current OS, we should pick the right target out of many,
  /// or return `None` if there is no good enough candidate
  pub fn pick<'a>(os: &'a client_os::Type, targets: &'a [Target]) -> Option<Target> {
    if targets.is_empty() {
      return None;
    }

    let mut candidates = targets.to_vec();
    candidates.sort_unstable();

    for candidate in &candidates {
      if candidate.is_compatible(os) {
        return Some(candidate.clone());
      }
    }

    None
  }

  pub fn is_compatible(&self, os: &client_os::Type) -> bool {
    use client_os::Type as Os;

    match (self, os) {
      (Target::Any, _) => true,
      (Target::Windows, Os::Windows) => true,
      (Target::Macos, Os::Macos) => true,
      (Target::Alpine, Os::Alpine) => true,
      (Target::Amazon, Os::Amazon) => true,
      (Target::Arch, Os::Arch) => true,
      (Target::Centos, Os::Centos) => true,
      (Target::Debian, Os::Debian) => true,
      (Target::Fedora, Os::Fedora) => true,
      (Target::Manjaro, Os::Manjaro) => true,
      (Target::OpenSUSE, Os::OpenSUSE) => true,
      (Target::OracleLinux, Os::OracleLinux) => true,
      (Target::Pop, Os::Pop) => true,
      (Target::Redhat, Os::Redhat) => true,
      (Target::RedHatEnterprise, Os::RedHatEnterprise) => true,
      (Target::Redox, Os::Redox) => true,
      (Target::Solus, Os::Solus) => true,
      (Target::SUSE, Os::SUSE) => true,
      (Target::Ubuntu, Os::Ubuntu) => true,

      (Target::Linux, _) => match os {
        Os::Macos | Os::Windows | Os::Unknown => false,
        _ => true,
      },

      _ => false,
    }
  }
}

impl PartialOrd for Target {
  fn partial_cmp(&self, other: &Target) -> Option<Ordering> {
    let sw = self.weight();
    let ow = other.weight();

    Some(sw.cmp(&ow))
  }
}

#[cfg(test)]
mod test_is_compatible_with {
  use super::client_os;
  use super::Target;
  use pretty_assertions::assert_eq;

  #[test]
  fn all_os_should_be_compatible_with_any() {
    let target = Target::Any;
    let all_os = client_os::Type::all();

    for os in all_os {
      let is_compatible = target.is_compatible(os);
      if !is_compatible {
        println!("os {:?} should be compatible with {:?}", os, target);
      }
      assert_eq!(is_compatible, true);
    }
  }

  #[test]
  fn same_targets_should_be_compatible_with_same_type_of_os() {
    let pairs = &[
      (Target::Linux, client_os::Type::Linux),
      (Target::Macos, client_os::Type::Macos),
      (Target::Windows, client_os::Type::Windows),
      (Target::Alpine, client_os::Type::Alpine),
      (Target::Amazon, client_os::Type::Amazon),
      (Target::Arch, client_os::Type::Arch),
      (Target::Centos, client_os::Type::Centos),
      (Target::Debian, client_os::Type::Debian),
      (Target::Fedora, client_os::Type::Fedora),
      (Target::Manjaro, client_os::Type::Manjaro),
      (Target::OpenSUSE, client_os::Type::OpenSUSE),
      (Target::OracleLinux, client_os::Type::OracleLinux),
      (Target::Pop, client_os::Type::Pop),
      (Target::Redhat, client_os::Type::Redhat),
      (Target::RedHatEnterprise, client_os::Type::RedHatEnterprise),
      (Target::Redox, client_os::Type::Redox),
      (Target::Solus, client_os::Type::Solus),
      (Target::SUSE, client_os::Type::SUSE),
      (Target::Ubuntu, client_os::Type::Ubuntu),
    ];

    for (target, os) in pairs {
      let is_compatible = target.is_compatible(os);
      if !is_compatible {
        println!(
          "target `{:?}` should be compatible with os `{:?}`",
          target, os
        );
      }
      assert_eq!(is_compatible, true);
    }
  }

  #[test]
  fn linux_distros_should_be_compatible_with_target_linux() {
    let pairs = &[
      (Target::Linux, client_os::Type::Alpine),
      (Target::Linux, client_os::Type::Amazon),
      (Target::Linux, client_os::Type::Arch),
      (Target::Linux, client_os::Type::Centos),
      (Target::Linux, client_os::Type::Debian),
      (Target::Linux, client_os::Type::Fedora),
      (Target::Linux, client_os::Type::Manjaro),
      (Target::Linux, client_os::Type::OpenSUSE),
      (Target::Linux, client_os::Type::OracleLinux),
      (Target::Linux, client_os::Type::Pop),
      (Target::Linux, client_os::Type::Redhat),
      (Target::Linux, client_os::Type::RedHatEnterprise),
      (Target::Linux, client_os::Type::Redox),
      (Target::Linux, client_os::Type::Solus),
      (Target::Linux, client_os::Type::SUSE),
      (Target::Linux, client_os::Type::Ubuntu),
    ];

    for (target, os) in pairs {
      let is_compatible = target.is_compatible(os);
      if !is_compatible {
        println!(
          "target `{:?}` should be compatible with os `{:?}`",
          target, os
        );
      }
      assert_eq!(is_compatible, true);
    }
  }

  #[test]
  fn other_platforms_should_not_be_compatible_with_linux() {
    let pairs = &[
      (Target::Linux, client_os::Type::Macos),
      (Target::Linux, client_os::Type::Windows),
      (Target::Linux, client_os::Type::Unknown),
    ];

    for (target, os) in pairs {
      let is_compatible = target.is_compatible(os);
      if is_compatible {
        println!(
          "target `{:?}` should not be compatible with os `{:?}`",
          target, os
        );
      }
      assert_eq!(is_compatible, false);
    }
  }
}

#[cfg(test)]
mod test_pick {
  use super::client_os;
  use super::Target;
  use pretty_assertions::assert_eq;

  #[test]
  fn should_return_none_when_no_targets_provided() {
    let targets: &[Target] = &[];
    let all_os = client_os::Type::all();

    for os in all_os {
      assert_eq!(Target::pick(os, targets), None);
    }
  }

  #[test]
  fn all_available_targets_should_pick_any_if_there_is_one() {
    let targets = &[Target::Any];
    let all_os = client_os::Type::all();

    for os in all_os {
      assert_eq!(Target::pick(os, targets), Some(Target::Any));
    }
  }

  #[test]
  fn all_available_targets_should_pick_any_if_there_is_one_amongst_others() {
    let targets = &[Target::Arch, Target::Any, Target::Ubuntu, Target::Macos];
    let all_os = client_os::Type::all();

    for os in all_os {
      assert_eq!(Target::pick(os, targets), Some(Target::Any));
    }
  }

  #[test]
  fn should_pick_the_same_os_as_the_target() {
    let targets = &[
      Target::Linux,
      Target::Macos,
      Target::Windows,
      Target::Alpine,
      Target::Amazon,
      Target::Arch,
      Target::Centos,
      Target::Debian,
      Target::Fedora,
      Target::Manjaro,
      Target::OpenSUSE,
      Target::OracleLinux,
      Target::Pop,
      Target::Redhat,
      Target::RedHatEnterprise,
      Target::Redox,
      Target::Solus,
      Target::SUSE,
      Target::Ubuntu,
    ];
    let pairs = &[
      (Target::Linux, client_os::Type::Linux),
      (Target::Macos, client_os::Type::Macos),
      (Target::Windows, client_os::Type::Windows),
      (Target::Alpine, client_os::Type::Alpine),
      (Target::Amazon, client_os::Type::Amazon),
      (Target::Arch, client_os::Type::Arch),
      (Target::Centos, client_os::Type::Centos),
      (Target::Debian, client_os::Type::Debian),
      (Target::Fedora, client_os::Type::Fedora),
      (Target::Manjaro, client_os::Type::Manjaro),
      (Target::OpenSUSE, client_os::Type::OpenSUSE),
      (Target::OracleLinux, client_os::Type::OracleLinux),
      (Target::Pop, client_os::Type::Pop),
      (Target::Redhat, client_os::Type::Redhat),
      (Target::RedHatEnterprise, client_os::Type::RedHatEnterprise),
      (Target::Redox, client_os::Type::Redox),
      (Target::Solus, client_os::Type::Solus),
      (Target::SUSE, client_os::Type::SUSE),
      (Target::Ubuntu, client_os::Type::Ubuntu),
    ];

    for (expected_target, os) in pairs {
      let target = Target::pick(os, targets);
      assert_eq!(&target.unwrap(), expected_target);
    }
  }

  #[test]
  fn distros_should_pick_linux() {
    let targets = &[Target::Macos, Target::Windows, Target::Linux];
    let distros = &[
      client_os::Type::Alpine,
      client_os::Type::Amazon,
      client_os::Type::Arch,
      client_os::Type::Centos,
      client_os::Type::Debian,
      client_os::Type::Fedora,
      client_os::Type::Manjaro,
      client_os::Type::OpenSUSE,
      client_os::Type::OracleLinux,
      client_os::Type::Pop,
      client_os::Type::Redhat,
      client_os::Type::RedHatEnterprise,
      client_os::Type::Redox,
      client_os::Type::Solus,
      client_os::Type::SUSE,
      client_os::Type::Ubuntu,
    ];

    for distro in distros {
      let target = Target::pick(distro, targets);
      assert_eq!(target.unwrap(), Target::Linux);
    }
  }
}
