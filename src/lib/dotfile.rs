use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct DotFile {
  pub name: String,
  pub from: PathBuf,
  pub to: PathBuf,
}

impl DotFile {
  /// Compiles final `to` path
  pub fn to(&self) -> PathBuf {
    PathBuf::from(&self.to).join(PathBuf::from(&self.name).file_name().unwrap())
  }

  /// Compiles final `from` path
  pub fn from(&self) -> PathBuf {
    PathBuf::from(&self.from).join(PathBuf::from(&self.name).file_name().unwrap())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn a01() {
    let file = DotFile {
      name: String::from("file.sh"),
      from: PathBuf::from("/from"),
      to: PathBuf::from("/to"),
    };

    assert_eq!(
      file.from(),
      PathBuf::from("/from/file.sh"),
      "should combine `from` and `name`"
    );
    assert_eq!(
      file.to(),
      PathBuf::from("/to/file.sh"),
      "should combine `to` and `name`"
    );
  }

  #[test]
  fn a02() {
    let file = DotFile {
      name: String::from("sub-folder/file.sh"),
      from: PathBuf::from("/from"),
      to: PathBuf::from("/to"),
    };

    assert_eq!(
      file.from(),
      PathBuf::from("/from/file.sh"),
      "should ignore path in the `name` if one exists"
    );
    assert_eq!(
      file.to(),
      PathBuf::from("/to/file.sh"),
      "should combine `to` and `name`"
    );
  }
}
