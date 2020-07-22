use crate::lib::{DotFile, Report};
use crate::Context;
use std::collections::HashMap;
use std::io::Result;

pub fn sudo<'a>(cx: &Context, dotfiles: &HashMap<u32, &DotFile>) -> Result<Vec<Report>> {
  let encoded = serde_json::to_string(dotfiles)?;

  let mut sudo = std::process::Command::new("sudo");
  sudo
    .arg("target/debug/dtf")
    .arg("ln")
    .arg(cx.config_path)
    .arg(format!("--dotfiles={}", encoded))
    .arg("--child=1");

  let res = sudo.output()?;

  let stdout = std::str::from_utf8(&res.stdout).unwrap();
  let reports: Vec<Report> = serde_json::from_str(stdout).unwrap();

  Ok(reports)
}
