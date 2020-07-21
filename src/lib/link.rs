use crate::lib::DotFile;
use crate::Context;
use std::fs;

#[derive(Debug)]
pub enum ErrorSource {
  Src,
  Dst,
}

#[derive(Debug)]
pub struct Error {
  kind: std::io::ErrorKind,
  source: ErrorSource,
}

pub fn link<'a>(
  cx: &Context,
  dotfile: &'a DotFile,
  rm_dst: Option<()>,
) -> Result<&'a DotFile<'a>, Error> {
  let src = &dotfile.src_file_path();
  let dst = &dotfile.dst_file_path();

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
    Ok(_) => Ok(dotfile),
    Err(e) => match e.kind() {
      std::io::ErrorKind::AlreadyExists => link(cx, dotfile, Some(())),
      _ => Err(Error {
        source: ErrorSource::Dst,
        kind: e.kind(),
      }),
    },
  }
}
