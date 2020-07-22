use crate::Context;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorSource {
  Src,
  Dst,
}

impl fmt::Display for ErrorSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ErrorSource::Src => write!(f, "source"),
      ErrorSource::Dst => write!(f, "destination"),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorKind {
  NotFound,
  PermissionDenied,
  AlreadyExists,
  Other,
}

impl fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

impl ErrorKind {
  pub fn as_str(&self) -> &str {
    match self {
      ErrorKind::NotFound => "not found",
      ErrorKind::PermissionDenied => "permission denied",
      ErrorKind::AlreadyExists => "already exists",
      ErrorKind::Other => "unknown error",
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  pub kind: ErrorKind,
  pub source: ErrorSource,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.source, self.kind)
  }
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
