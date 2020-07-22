use crate::lib::{dotfile, sudo, DotFile, Report};
use crate::Context;
use std::collections::HashMap;
use std::io::Result;

pub fn link(cx: &Context, dotfiles: &HashMap<u32, DotFile>) -> Result<()> {
  let ref mut denied: HashMap<u32, &DotFile> = HashMap::new();
  let mut reports: Vec<Report> = Vec::with_capacity(dotfiles.len());

  for (id, dotfile) in dotfiles {
    match dotfile.link(cx, None) {
      Ok(_) => reports.push(Report {
        dotfile_id: *id,
        error: None,
      }),
      Err(e) => match e.kind {
        dotfile::ErrorKind::PermissionDenied => {
          denied.insert(*id, dotfile);
        }
        _ => reports.push(Report {
          dotfile_id: *id,
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

    for report in &reports {
      if report.is_ok() {
        continue;
      }

      let dotfile = dotfiles.get(&report.dotfile_id).unwrap();
      let err = report.error.as_ref().unwrap();

      Report::print(dotfile, err);
    }

    Ok(())
  } else {
    let stdout = serde_json::to_string(&reports).unwrap();
    println!("{}", stdout);
    Ok(())
  }
}
