use crate::Report;
use dtflib::{DotFile, CHILD_PARAM};
use std::collections::HashMap;
use std::io::prelude::{Read, Write};
use std::io::Result;
use std::process::{Command, Stdio};

pub fn sudo<'a>(dotfiles: &HashMap<u32, &DotFile>) -> Result<Vec<Report>> {
  let sudo = Command::new("sudo")
    .args(std::env::args())
    .arg(CHILD_PARAM)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  let dotfiles_json = serde_json::to_string(dotfiles)?;
  sudo.stdin.unwrap().write_all(dotfiles_json.as_bytes())?;

  let mut reports_json = String::new();
  sudo.stdout.unwrap().read_to_string(&mut reports_json)?;
  let reports: Vec<Report> = serde_json::from_str(&reports_json).unwrap();

  Ok(reports)
}
