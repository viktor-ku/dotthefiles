use crate::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorSource {
  Src,
  Dst,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorKind {
  NotFound,
  PermissionDenied,
  AlreadyExists,
  Other,
}

impl std::convert::From<std::io::ErrorKind> for ErrorKind {
  fn from(io_err_kind: std::io::ErrorKind) -> Self {
    match &io_err_kind {
      std::io::ErrorKind::NotFound => Self::NotFound,
      std::io::ErrorKind::PermissionDenied => Self::PermissionDenied,
      std::io::ErrorKind::AlreadyExists => Self::AlreadyExists,
      _ => Self::Other,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  pub kind: ErrorKind,
  pub source: ErrorSource,
}

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

  pub fn link(&self, cx: &Context, rm_dst: Option<()>) -> Result<(), Error> {
    let src = &self.src_file_path();
    let dst = &self.dst_file_path();

    if rm_dst.is_some() {
      match fs::remove_file(dst) {
        Ok(_) => {}
        Err(e) => {
          return Err(Error {
            source: ErrorSource::Src,
            kind: e.kind().into(),
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
          kind: e.kind().into(),
        }),
      },
    }
  }
}
