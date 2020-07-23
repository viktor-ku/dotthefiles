use crate::Context;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorKind {
  NotFound,
  PermissionDenied,
  AlreadyExists,
  Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorStage {
  RemoveFile,
  HardLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  pub kind: ErrorKind,
  pub stage: ErrorStage,
  pub message: String,
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
            kind: e.kind().into(),
            message: e.to_string(),
            stage: ErrorStage::RemoveFile,
          })
        }
      }
    }

    match fs::hard_link(src, dst) {
      Ok(_) => Ok(()),
      Err(e) => match e.kind() {
        std::io::ErrorKind::AlreadyExists => self.link(cx, Some(())),
        std::io::ErrorKind::NotFound => Err(Error {
          kind: e.kind().into(),
          message: "source file was not found".to_owned(),
          stage: ErrorStage::HardLink,
        }),
        _ => Err(Error {
          kind: e.kind().into(),
          message: e.to_string(),
          stage: ErrorStage::HardLink,
        }),
      },
    }
  }
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

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl fmt::Display for ErrorStage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ErrorStage::RemoveFile => write!(f, "remove destination file"),
      ErrorStage::HardLink => write!(f, "make a hard link"),
    }
  }
}
