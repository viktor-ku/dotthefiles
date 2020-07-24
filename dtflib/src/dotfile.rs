use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DotFile<'a> {
  pub id: u32,
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
