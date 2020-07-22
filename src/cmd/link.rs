use crate::lib::{dotfile, DotFile};
use crate::Context;
use std::io::Result;

pub fn link(cx: &Context, dotfiles: &Vec<DotFile>) -> Result<()> {
  let ref mut denied: Vec<&DotFile> = Vec::with_capacity(dotfiles.len());

  for dotfile in dotfiles {
    match dotfile.link(cx, None) {
      Ok(_) => {}
      Err(e) => match e.kind {
        dotfile::ErrorKind::PermissionDenied => {
          denied.push(dotfile);
        }
        _ => {
          // oh well?
        }
      },
    }
  }

  if !denied.is_empty() {
    let denied_json = serde_json::to_string(&denied)?;

    let mut sudo = std::process::Command::new("sudo");

    sudo
      .arg("target/debug/dtf")
      .arg("ln")
      .arg(cx.config_path)
      .arg(format!("--dotfiles={}", denied_json));

    let res = sudo.output()?;
  }

  Ok(())
}
