use crate::{hard_link, sudo, Report};
use dtflib::{Context, DotFile};
use std::collections::HashMap;
use std::io::Result;

pub fn link(cx: &Context, dotfiles: &HashMap<u32, DotFile>, force: bool) -> Result<()> {
  let ref mut denied: HashMap<u32, &DotFile> = HashMap::new();
  let mut reports: Vec<Report> = Vec::with_capacity(dotfiles.len());

  for (id, dotfile) in dotfiles {
    match hard_link::hard_link(cx, dotfile, force, None) {
      Ok(_) => reports.push(Report {
        dotfile_id: *id,
        error: None,
      }),
      Err(e) => match e.kind {
        hard_link::ErrorKind::PermissionDenied => {
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
      let sreports = sudo(denied)?;
      reports.extend(sreports);
    }

    let mut exit = 0;

    for report in &reports {
      if report.is_ok() {
        continue;
      }

      exit = 1;

      let dotfile = dotfiles.get(&report.dotfile_id).unwrap();
      let err = report.error.as_ref().unwrap();

      Report::print(dotfile, err);
    }

    if exit == 1 {
      std::process::exit(1);
    }

    Ok(())
  } else {
    let stdout = serde_json::to_string(&reports).unwrap();
    println!("{}", stdout);
    Ok(())
  }
}
