use crate::lib::{client_os, config, DotFile, Render, RenderState};
use crate::Context;
use std::io;

/// Should return the right directory name to get our files from (pan intended).
/// Based on the target and currently used OS.
fn source_dir<'a>(cx: &Context, target: &config::Target) -> Option<&'a str> {
  match cx.client_os {
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

pub fn map<'a>(cx: &Context, config: &'a config::Config) -> io::Result<Vec<DotFile<'a>>> {
  let mut v: Vec<DotFile> = Vec::with_capacity(32);

  for section in &config.map {
    let target: &config::Target = {
      let mut compatible: Vec<&config::Target> = section
        .target
        .iter()
        .filter(|target| target == &cx.client_os)
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
        home_dir: &cx.home_dir,
        base_dir: &cx.base_dir,
        source_dir: &source_dir(cx, target),
      };

      v.push(DotFile {
        name: &file.name,
        dst: to.render(&state),
        src: from.render(&state),
      })
    }
  }

  Ok(v)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lib::{read_yaml, User};
  use pretty_assertions::assert_eq;
  use std::path::PathBuf;

  fn base_dir(t: &str) -> PathBuf {
    std::env::current_dir().unwrap().join("examples").join(t)
  }

  fn user() -> User {
    User {
      uid: 101,
      euid: 101,
      gid: 20,
    }
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Darwin,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "ide-script.sh",
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

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
      config_path,
      user: &user(),
    };

    let actual = map(&cx, &config)?;

    let expected: Vec<DotFile> = vec![DotFile {
      name: "file.sh",
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

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        user: &user(),
      };

      let actual = map(&cx, &config)?;

      let expected: Vec<DotFile> = vec![DotFile {
        name: "file.sh",
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

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        user: &user(),
      };

      let actual = map(&cx, &config)?;

      let expected: Vec<DotFile> = vec![DotFile {
        name: "file.sh",
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

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        user: &user(),
      };

      let actual = map(&cx, &config)?;

      let expected: Vec<DotFile> = vec![DotFile {
        name: "file.sh",
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

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        user: &user(),
      };

      let actual = map(&cx, &config)?;

      let expected: Vec<DotFile> = vec![DotFile {
        name: "file.sh",
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

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        user: &user(),
      };

      let actual = map(&cx, &config)?;

      let expected: Vec<DotFile> = vec![DotFile {
        name: "file.sh",
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

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        user: &user(),
      };

      let actual = map(&cx, &config)?;

      let expected: Vec<DotFile> = vec![DotFile {
        name: "file.sh",
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
