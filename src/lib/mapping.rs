use crate::lib::{client_os, config, DotFile, Render, RenderState};
use std::io;
use std::path::PathBuf;

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
        let from = Render::from(&section.from);

        let state = RenderState {
          home_dir: &self.home_dir,
          base_dir: &self.base_dir,
          source_dir: &self.source_dir(target),
        };

        v.push(DotFile {
          name: file.name.clone(),
          dst: to.render(&state),
          src: from.render(&state),
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
      src: PathBuf::from(&base_dir.join("files/linux")),
      dst: PathBuf::from(&home_dir),
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
      src: PathBuf::from(&base_dir.join("files/darwin")),
      dst: PathBuf::from(&home_dir),
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
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from(&home_dir),
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
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from(&home_dir),
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
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from(&home_dir),
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
      src: PathBuf::from(&base_dir.join("files/darwin")),
      dst: PathBuf::from(&home_dir),
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
      src: PathBuf::from(&base_dir.join("files/linux")),
      dst: PathBuf::from(&home_dir).join("Code"),
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
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from("/etc/some"),
    }];

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual, expected,
      "should decide to link from files/ to /etc/some"
    );

    Ok(())
  }

  mod section_based_from {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a11() -> io::Result<()> {
      let base_dir = &base_dir("a11");
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
        src: PathBuf::from(&base_dir.join("otherstuff")),
        dst: PathBuf::from(&home_dir).join("some"),
      }];

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, expected,
        "if `from` field is provided and is a relative path then resolve it relative to the config's location regardless of the client os"
      );

      Ok(())
    }

    #[test]
    fn a12() -> io::Result<()> {
      let base_dir = &base_dir("a12");
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
        src: PathBuf::from(&base_dir).join("otherstuff"),
        dst: PathBuf::from(&home_dir).join("some"),
      }];

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, expected,
        "should resolve relative `from` regardless of the target os"
      );

      Ok(())
    }

    #[test]
    fn a13() -> io::Result<()> {
      let base_dir = &base_dir("a13");
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
        src: PathBuf::from(&home_dir).join("backup"),
        dst: PathBuf::from(&home_dir).join("some"),
      }];

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, expected,
        "should be able to resolve home dir correctly in the `from` field"
      );

      Ok(())
    }

    #[test]
    fn a14() -> io::Result<()> {
      let base_dir = &base_dir("a14");
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
        src: PathBuf::from("/my/bucket/with/stuff/by/linux"),
        dst: PathBuf::from(&home_dir).join("some"),
      }];

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, expected,
        "should look for the absolute path with target folder when a $TARGET variable is present"
      );

      Ok(())
    }

    #[test]
    fn a15() -> io::Result<()> {
      let base_dir = &base_dir("a15");
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
        src: PathBuf::from("/my/bucket/with/stuff/by"),
        dst: PathBuf::from(&home_dir).join("some"),
      }];

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, expected,
        "in case there is a $TARGET variable in the `from` field, but the target is any, should look for just the `from` field"
      );

      Ok(())
    }

    #[test]
    fn a16() -> io::Result<()> {
      let base_dir = &base_dir("a16");
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
        src: PathBuf::from(&base_dir).join("stuff"),
        dst: PathBuf::from(&home_dir).join("some"),
      }];

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, expected,
        "should just look into the /stuff folder for any file for any platform"
      );

      Ok(())
    }
  }
}
