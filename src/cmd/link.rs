use crate::lib::{dotfile, sudo, DotFile, Report};
use crate::Context;
use std::io::Result;

pub fn link(cx: &Context, dotfiles: &Vec<DotFile>) -> Result<()> {
  let ref mut denied: Vec<&DotFile> = Vec::with_capacity(dotfiles.len());
  let mut reports: Vec<Report> = Vec::with_capacity(dotfiles.len());

  for dotfile in dotfiles {
    match dotfile.link(cx, None) {
      Ok(_) => reports.push(Report {
        dotfile: dotfile.id,
        error: None,
      }),
      Err(e) => match e.kind {
        dotfile::ErrorKind::PermissionDenied => {
          denied.push(dotfile);
        }
        _ => reports.push(Report {
          dotfile: dotfile.id,
          error: Some(e),
        }),
      },
    }
  }

  if cx.is_main() {
    if !denied.is_empty() {
      let sreports = sudo(cx, denied)?;
      reports.extend(sreports);
    }

    println!("reports {:#?}", reports);

    Ok(())
  } else {
    let stdout = serde_json::to_string(&reports).unwrap();
    println!("{}", stdout);
    Ok(())
  }
}
