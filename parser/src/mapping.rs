use crate::{
  config::{Config, Target},
  Render, RenderState,
};
use dtflib::{Context, DotFile};
use std::collections::HashMap;
use std::io::Result;

pub fn map<'a>(cx: &Context, config: &'a Config) -> Result<HashMap<u32, DotFile<'a>>> {
  let mut id: u32 = 0;
  let mut ret: HashMap<u32, DotFile<'a>> = HashMap::new();

  for section in &config.map {
    let target = Target::pick(cx.client_os, &section.target);

    if target.is_none() {
      continue;
    }

    let target = target.unwrap();

    for file in &section.files {
      let to = Render::from(&file.to);
      let from = Render::from(&section.from);

      let state = RenderState {
        home_dir: &cx.home_dir,
        base_dir: &cx.base_dir,
        source_dir: target.dir(),
      };

      id += 1;

      let dotfile = DotFile {
        id,
        name: &file.name,
        dst: to.render(&state),
        src: from.render(&state),
      };

      ret.insert(id, dotfile);
    }
  }

  Ok(ret)
}

#[cfg(test)]
mod tests {
  use super::map;
  use crate::read_file;
  use dtflib::{client_os, Context, DotFile};
  use pretty_assertions::assert_eq;
  use std::collections::HashMap;
  use std::io;
  use std::path::PathBuf;

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

  #[inline]
  fn to_map(expected: DotFile) -> HashMap<u32, DotFile> {
    let mut val = HashMap::new();
    val.insert(expected.id, expected);
    val
  }

  #[test]
  fn a01() -> io::Result<()> {
    let base_dir = &base_dir("a01");
    let home_dir = &FakeHomeDir::linux();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files/linux")),
      dst: PathBuf::from(&home_dir),
    };

    assert_eq!(
      actual,
      to_map(expected),
      "given the right target it should provide us with the simplest file mapping"
    );

    Ok(())
  }

  #[test]
  fn a02() -> io::Result<()> {
    let base_dir = &base_dir("a02");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;
    let expected = HashMap::new();

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

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = HashMap::new();

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

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files/macos")),
      dst: PathBuf::from(&home_dir),
    };

    assert_eq!(
      actual,
      to_map(expected),
      "should set right os type into the 'from' field"
    );

    Ok(())
  }

  #[test]
  fn a05() -> io::Result<()> {
    let base_dir = &base_dir("a05");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from(&home_dir),
    };

    assert_eq!(
      actual,
      to_map(expected),
      "should read 'any' target correctly"
    );

    Ok(())
  }

  #[test]
  fn a06() -> io::Result<()> {
    let base_dir = &base_dir("a06");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from(&home_dir),
    };

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual,
      to_map(expected),
      "when `target` contains `any` in it alongside with other targets, treat it like `any` anyway"
    );

    Ok(())
  }

  #[test]
  fn a07() -> io::Result<()> {
    let base_dir = &base_dir("a07");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from(&home_dir),
    };

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual,
      to_map(expected),
      "when there is no `target` defined, treat it like `any` by default"
    );

    Ok(())
  }

  #[test]
  fn a08() -> io::Result<()> {
    let base_dir = &base_dir("a08");
    let home_dir = &FakeHomeDir::darwin();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Macos,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files/macos")),
      dst: PathBuf::from(&home_dir),
    };

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual,
      to_map(expected),
      "should pick the right one out of two"
    );

    Ok(())
  }

  #[test]
  fn a09() -> io::Result<()> {
    let base_dir = &base_dir("a09");
    let home_dir = &FakeHomeDir::linux();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "ide-script.sh",
      src: PathBuf::from(&base_dir.join("files/linux")),
      dst: PathBuf::from(&home_dir).join("Code"),
    };

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual,
      to_map(expected),
      "should pick the right one out of two"
    );

    Ok(())
  }

  #[test]
  fn a10() -> io::Result<()> {
    let base_dir = &base_dir("a10");
    let home_dir = &FakeHomeDir::linux();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_file(config_path)?;

    let cx = Context {
      base_dir,
      home_dir: &home_dir,
      client_os: &client_os::Type::Linux,
      config_path,
      child: true,
    };

    let actual = map(&cx, &config)?;

    let expected = DotFile {
      id: 1,
      name: "file.sh",
      src: PathBuf::from(&base_dir.join("files")),
      dst: PathBuf::from("/etc/some"),
    };

    println!("\n|> {:}\n", &config_path.to_str().unwrap());

    assert_eq!(
      actual,
      to_map(expected),
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

      let config = read_file(config_path)?;

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        child: true,
      };

      let actual = map(&cx, &config)?;

      let expected = DotFile {
        id: 1,
        name: "file.sh",
        src: PathBuf::from(&base_dir.join("otherstuff")),
        dst: PathBuf::from(&home_dir).join("some"),
      };

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, to_map(expected),
        "if `from` field is provided and is a relative path then resolve it relative to the config's location regardless of the client os"
      );

      Ok(())
    }

    #[test]
    fn a12() -> io::Result<()> {
      let base_dir = &base_dir("a12");
      let home_dir = &FakeHomeDir::linux();
      let config_path = &base_dir.join("dotthefiles.yml");

      let config = read_file(config_path)?;

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        child: true,
      };

      let actual = map(&cx, &config)?;

      let expected = DotFile {
        id: 1,
        name: "file.sh",
        src: PathBuf::from(&base_dir).join("otherstuff"),
        dst: PathBuf::from(&home_dir).join("some"),
      };

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual,
        to_map(expected),
        "should resolve relative `from` regardless of the target os"
      );

      Ok(())
    }

    #[test]
    fn a13() -> io::Result<()> {
      let base_dir = &base_dir("a13");
      let home_dir = &FakeHomeDir::linux();
      let config_path = &base_dir.join("dotthefiles.yml");

      let config = read_file(config_path)?;

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        child: true,
      };

      let actual = map(&cx, &config)?;

      let expected = DotFile {
        id: 1,
        name: "file.sh",
        src: PathBuf::from(&home_dir).join("backup"),
        dst: PathBuf::from(&home_dir).join("some"),
      };

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual,
        to_map(expected),
        "should be able to resolve home dir correctly in the `from` field"
      );

      Ok(())
    }

    #[test]
    fn a14() -> io::Result<()> {
      let base_dir = &base_dir("a14");
      let home_dir = &FakeHomeDir::linux();
      let config_path = &base_dir.join("dotthefiles.yml");

      let config = read_file(config_path)?;

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        child: true,
      };

      let actual = map(&cx, &config)?;

      let expected = DotFile {
        id: 1,
        name: "file.sh",
        src: PathBuf::from("/my/bucket/with/stuff/by/linux"),
        dst: PathBuf::from(&home_dir).join("some"),
      };

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual,
        to_map(expected),
        "should look for the absolute path with target folder when a $TARGET variable is present"
      );

      Ok(())
    }

    #[test]
    fn a15() -> io::Result<()> {
      let base_dir = &base_dir("a15");
      let home_dir = &FakeHomeDir::linux();
      let config_path = &base_dir.join("dotthefiles.yml");

      let config = read_file(config_path)?;

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        child: true,
      };

      let actual = map(&cx, &config)?;

      let expected = DotFile {
        id: 1,
        name: "file.sh",
        src: PathBuf::from("/my/bucket/with/stuff/by"),
        dst: PathBuf::from(&home_dir).join("some"),
      };

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual, to_map(expected),
        "in case there is a $TARGET variable in the `from` field, but the target is any, should look for just the `from` field"
      );

      Ok(())
    }

    #[test]
    fn a16() -> io::Result<()> {
      let base_dir = &base_dir("a16");
      let home_dir = &FakeHomeDir::linux();
      let config_path = &base_dir.join("dotthefiles.yml");

      let config = read_file(config_path)?;

      let cx = Context {
        base_dir,
        home_dir: &home_dir,
        client_os: &client_os::Type::Linux,
        config_path,
        child: true,
      };

      let actual = map(&cx, &config)?;

      let expected = DotFile {
        id: 1,
        name: "file.sh",
        src: PathBuf::from(&base_dir).join("stuff"),
        dst: PathBuf::from(&home_dir).join("some"),
      };

      println!("\n|> {:}\n", &config_path.to_str().unwrap());

      assert_eq!(
        actual,
        to_map(expected),
        "should just look into the /stuff folder for any file for any platform"
      );

      Ok(())
    }
  }
}
