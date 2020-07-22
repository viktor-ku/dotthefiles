use crate::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ErrorSource {
  Src,
  Dst,
}

#[derive(Debug)]
pub struct Error {
  pub kind: std::io::ErrorKind,
  pub source: ErrorSource,
}

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

  pub fn link(&self, cx: &Context, rm_dst: Option<()>) -> Result<(), Error> {
    let src = &self.src_file_path();
    let dst = &self.dst_file_path();

    if rm_dst.is_some() {
      match fs::remove_file(dst) {
        Ok(_) => {}
        Err(e) => {
          return Err(Error {
            source: ErrorSource::Src,
            kind: e.kind(),
          })
        }
      }
    }

    match fs::hard_link(src, dst) {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::AlreadyExists => self.link(cx, Some(())),
        _ => Err(Error {
          source: ErrorSource::Dst,
          kind: e.kind(),
        }),
      },
    }
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
