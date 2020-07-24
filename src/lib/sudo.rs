use crate::lib::{DotFile, Report};
use crate::Context;
use std::collections::HashMap;
use std::io::prelude::{Read, Write};
use std::io::Result;
use std::process::{Command, Stdio};

pub fn sudo<'a>(cx: &Context, dotfiles: &HashMap<u32, &DotFile>) -> Result<Vec<Report>> {
  let dotfiles_json = serde_json::to_string(dotfiles)?;

  let sudo = Command::new("sudo")
    .arg("target/debug/dtf")
    .arg("ln")
    .arg(cx.config_path)
    .arg("--child=1")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

  sudo.stdin.unwrap().write_all(dotfiles_json.as_bytes())?;

  let mut reports_json = String::new();
  sudo.stdout.unwrap().read_to_string(&mut reports_json)?;
  let reports: Vec<Report> = serde_json::from_str(&reports_json).unwrap();

  Ok(reports)
}
