use crate::lib::DotFile;
use crate::Context;
use std::io::Result;

pub fn link(cx: &Context, dotfiles: &Vec<DotFile>) -> Result<()> {
  let ref mut denied: Vec<&DotFile> = Vec::with_capacity(dotfiles.len());

  for dotfile in dotfiles {
    match dotfile.link(cx, None) {
      Ok(_) => {},
      Err(e) => match e.kind {
        std::io::ErrorKind::PermissionDenied => {
          denied.push(dotfile);
        }
        _ => {
          // oh well?
          println!("error {:#?} for {:#?}", e, dotfile);
        }
      }
    }
  }

  if !denied.is_empty() {
    println!("denied {:#?}", denied);


    let mut sudo = std::process::Command::new("sudo");

    let denied_json = serde_json::to_string(&denied)?;

    sudo
      .arg("target/debug/dtf")
      .arg("ln")
      .arg(cx.config_path)
      .arg(format!("--dotfiles={}", denied_json));

    let res = sudo.output()?;


    println!("{:#?}", sudo);
    println!("{:#?}", res);
  }

  Ok(())
}
