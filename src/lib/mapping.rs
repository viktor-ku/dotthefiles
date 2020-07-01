use crate::lib::config;
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct DotFile {
  name: String,
  from: PathBuf,
  to: PathBuf,
}

#[derive(Debug)]
pub struct Mapping<'a> {
  pub base_dir: &'a PathBuf,
  pub os_type: &'a os_info::Type,
  pub home_dir: &'a PathBuf,
}

impl<'a> Mapping<'a> {
  /// Should return the right directory name to get our files from (pan intended).
  /// Based on the target and currently used OS.
  pub fn source_dir(&self, target: &config::Target) -> &str {
    match self.os_type {
      os_info::Type::Linux => {
        if target == &config::Target::Any {
          "any"
        } else {
          "linux"
        }
      }
      os_info::Type::Macos => {
        if target == &config::Target::Any {
          "any"
        } else {
          "darwin"
        }
      }
      os_info::Type::Windows => {
        if target == &config::Target::Any {
          "any"
        } else {
          "win"
        }
      }
      _ => panic!("do not know which source directory to use!"),
    }
  }

  pub fn map(&self, config: &config::Config) -> io::Result<Vec<DotFile>> {
    let mut v: Vec<DotFile> = Vec::with_capacity(32);

    for section in &config.map {
      let mut compatible: Option<&config::Target> = None;

      for target in &section.target {
        if target == self.os_type {
          compatible = Some(target);
          break;
        }
      }

      if compatible.is_none() {
        continue;
      }

      let compatible = compatible.unwrap();

      for file in &section.files {
        let to: PathBuf = if file.to == "~/" {
          self.home_dir.clone()
        } else {
          PathBuf::from(&file.to)
        };

        let from = PathBuf::from(self.base_dir)
          .join("files")
          .join(self.source_dir(compatible));

        v.push(DotFile {
          name: file.name.clone(),
          to,
          from,
        })
      }
    }

    Ok(v)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lib::read_yaml;
  use pretty_assertions::assert_eq;

  fn base_dir(t: &str) -> PathBuf {
    std::env::current_dir().unwrap().join("examples").join(t)
  }

  #[tokio::test]
  async fn a01() -> io::Result<()> {
    let os_type = &os_info::Type::Linux;
    let base_dir = &base_dir("a01");
    let home_dir = &dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files/linux")),
      to: PathBuf::from(&home_dir),
    }];

    assert_eq!(
      actual, expected,
      "given the right target it should provide us with the simplest file mapping"
    );

    Ok(())
  }

  #[tokio::test]
  async fn a02() -> io::Result<()> {
    let os_type = &os_info::Type::Macos;
    let base_dir = &base_dir("a02");
    let home_dir = &dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![];

    assert_eq!(
      actual, expected,
      "If we are using undeclared OS then return nothing"
    );

    Ok(())
  }

  #[tokio::test]
  async fn a03() -> io::Result<()> {
    let os_type = &os_info::Type::Macos;
    let base_dir = &base_dir("a03");
    let home_dir = &dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![];

    assert_eq!(
      actual, expected,
      "if there is an empty targets vector then we ignore it"
    );

    Ok(())
  }

  #[tokio::test]
  async fn a04() -> io::Result<()> {
    let os_type = &os_info::Type::Macos;
    let base_dir = &base_dir("a04");
    let home_dir = &dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files/darwin")),
      to: PathBuf::from(&home_dir),
    }];

    assert_eq!(
      actual, expected,
      "should set right os type into the 'from' field"
    );

    Ok(())
  }

  #[tokio::test]
  async fn a05() -> io::Result<()> {
    let os_type = &os_info::Type::Macos;
    let base_dir = &base_dir("a05");
    let home_dir = &dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files/any")),
      to: PathBuf::from(&home_dir),
    }];

    assert_eq!(
      actual, expected,
      "should read 'any' target correctly and pass the file in to darwin"
    );

    Ok(())
  }
}
