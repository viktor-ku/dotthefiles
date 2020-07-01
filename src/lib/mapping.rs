use crate::lib::config::Config;
use crate::lib::os_type_to_string;
use async_std::io;
use async_std::path::PathBuf;

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
  pub async fn map(&self, config: &Config) -> io::Result<Vec<DotFile>> {
    let mut v: Vec<DotFile> = Vec::with_capacity(32);

    for section in &config.map {
      let mut compatible = false;

      for target in &section.target {
        if target == self.os_type {
          compatible = true;
          break;
        }
      }

      if !compatible {
        continue;
      }

      for file in &section.files {
        let to: PathBuf = if file.to == "~/" {
          self.home_dir.clone()
        } else {
          PathBuf::from(&file.to)
        };

        let from = PathBuf::from(self.base_dir)
          .join("files")
          .join(os_type_to_string(&self.os_type));

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
    PathBuf::from(std::env::current_dir().unwrap())
      .join("examples")
      .join(t)
  }

  #[tokio::test]
  async fn a01() -> io::Result<()> {
    let os_type = &os_info::Type::Linux;
    let base_dir = &base_dir("a01");
    let home_dir = &dirs::home_dir().unwrap();
    let config_path = &base_dir.join("dotthefiles.yml");

    let config = read_yaml(config_path).await?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config).await?;

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

    let config = read_yaml(config_path).await?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config).await?;

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

    let config = read_yaml(config_path).await?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config).await?;

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

    let config = read_yaml(config_path).await?;

    let mapping = Mapping {
      base_dir,
      os_type: &os_type,
      home_dir: &home_dir.into(),
    };

    let actual = mapping.map(&config).await?;

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
}
