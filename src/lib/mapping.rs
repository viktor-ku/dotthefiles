use crate::lib::{client_os, config, Render};
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
  pub client_os: &'a client_os::Type,
  pub home_dir: &'a PathBuf,
}

impl<'a> Mapping<'a> {
  /// Should return the right directory name to get our files from (pan intended).
  /// Based on the target and currently used OS.
  pub fn source_dir(&self, target: &config::Target) -> Option<&str> {
    match self.client_os {
      client_os::Type::Linux => {
        if target == &config::Target::Any {
          None
        } else {
          Some("linux")
        }
      }
      client_os::Type::Darwin => {
        if target == &config::Target::Any {
          None
        } else {
          Some("darwin")
        }
      }
      client_os::Type::Win => {
        if target == &config::Target::Any {
          None
        } else {
          Some("win")
        }
      }
      _ => panic!("do not know which source directory to use!"),
    }
  }

  pub fn map(&self, config: &config::Config) -> io::Result<Vec<DotFile>> {
    let mut v: Vec<DotFile> = Vec::with_capacity(32);

    for section in &config.map {
      let target: &config::Target = {
        let mut compatible: Vec<&config::Target> = section
          .target
          .iter()
          .filter(|target| target == &self.client_os)
          .collect();

        if compatible.is_empty() {
          continue;
        }

        compatible.sort_unstable();

        *compatible.first().unwrap()
      };

      for file in &section.files {
        let to = Render::from(&file.to);

        let mut from = PathBuf::from(self.base_dir).join("files");

        if let Some(source_dir) = self.source_dir(target) {
          from.push(source_dir);
        }

        v.push(DotFile {
          name: file.name.clone(),
          to: to.render(self.home_dir),
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

  struct FakeHomeDir;

  impl FakeHomeDir {
    fn linux() -> PathBuf {
      PathBuf::from("/home/gman")
    }
    fn darwin() -> PathBuf {
      PathBuf::from("/Users/gman")
    }
  }

  #[test]
  fn a01() -> io::Result<()> {
    let base_dir = &base_dir("a01");
    let home_dir = &FakeHomeDir::linux();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
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

  #[test]
  fn a02() -> io::Result<()> {
    let base_dir = &base_dir("a02");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![];

    assert_eq!(
      actual, expected,
      "If we are using undeclared OS then return nothing"
    );

    Ok(())
  }

  #[test]
  fn a03() -> io::Result<()> {
    let base_dir = &base_dir("a03");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![];

    assert_eq!(
      actual, expected,
      "if there is an empty targets vector then we ignore it"
    );

    Ok(())
  }

  #[test]
  fn a04() -> io::Result<()> {
    let base_dir = &base_dir("a04");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
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

  #[test]
  fn a05() -> io::Result<()> {
    let base_dir = &base_dir("a05");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files")),
      to: PathBuf::from(&home_dir),
    }];

    assert_eq!(actual, expected, "should read 'any' target correctly");

    Ok(())
  }

  #[test]
  fn a06() -> io::Result<()> {
    let base_dir = &base_dir("a06");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files")),
      to: PathBuf::from(&home_dir),
    }];

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual, expected,
      "when `target` contains `any` in it alongside with other targets, treat it like `any` anyway"
    );

    Ok(())
  }

  #[test]
  fn a07() -> io::Result<()> {
    let base_dir = &base_dir("a07");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files")),
      to: PathBuf::from(&home_dir),
    }];

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual, expected,
      "when there is no `target` defined, treat it like `any` by default"
    );

    Ok(())
  }

  #[test]
  fn a08() -> io::Result<()> {
    let base_dir = &base_dir("a08");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files/darwin")),
      to: PathBuf::from(&home_dir),
    }];

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(actual, expected, "should pick the right one out of two");

    Ok(())
  }

  #[test]
  fn a09() -> io::Result<()> {
    let base_dir = &base_dir("a09");
    let home_dir = &FakeHomeDir::linux();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("ide-script.sh"),
      from: PathBuf::from(&base_dir.join("files/linux")),
      to: PathBuf::from(&home_dir).join("Code"),
    }];

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(actual, expected, "should pick the right one out of two");

    Ok(())
  }

  #[test]
  fn a10() -> io::Result<()> {
    let base_dir = &base_dir("a10");
    let home_dir = &FakeHomeDir::linux();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path)?;

    let mapping = Mapping {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
    };

    let actual = mapping.map(&config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from(&base_dir.join("files")),
      to: PathBuf::from("/etc/some"),
    }];

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(actual, expected, "should decide to link from files/ to /etc/some");

    Ok(())
  }
}
