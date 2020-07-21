use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct DotFile<'a> {
  pub name: &'a str,
  pub src: PathBuf,
  pub dst: PathBuf,
}

impl<'a> DotFile<'a> {
  /// Compiles final `to` path
  pub fn to(&self) -> PathBuf {
    PathBuf::from(&self.dst).join(PathBuf::from(&self.name).file_name().unwrap())
  }

  /// Compiles final `from` path
  pub fn from(&self) -> PathBuf {
    PathBuf::from(&self.src).join(PathBuf::from(&self.name).file_name().unwrap())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn a01() {
    let file = DotFile {
      name: "file.sh",
      src: PathBuf::from("/from"),
      dst: PathBuf::from("/to"),
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
      name: "sub-folder/file.sh",
      src: PathBuf::from("/from"),
      dst: PathBuf::from("/to"),
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
