use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DotFile<'a> {
  pub name: &'a str,
  pub src: PathBuf,
  pub dst: PathBuf,
}

impl<'a> DotFile<'a> {
  /// Compiles final `to` path
  pub fn dst_file_path(&self) -> PathBuf {
    PathBuf::from(&self.dst).join(self.name)
  }

  /// Compiles final `from` path
  pub fn src_file_path(&self) -> PathBuf {
    PathBuf::from(&self.src).join(self.name)
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
      file.src_file_path(),
      PathBuf::from("/from/file.sh"),
      "should combine `from` and `name`"
    );
    assert_eq!(
      file.dst_file_path(),
      PathBuf::from("/to/file.sh"),
      "should combine `to` and `name`"
    );
  }
}
