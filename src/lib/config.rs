use serde::Deserialize;
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Target {
  #[serde(alias = "linux")]
  Linux,

  #[serde(alias = "darwin", alias = "macos")]
  Darwin,

  #[serde(alias = "win", alias = "windows")]
  Win,

  #[serde(
    alias = "*",
    alias = "any",
    alias = "all",
    alias = "every",
    alias = "each"
  )]
  Any,
}

impl PartialEq<os_info::Type> for Target {
  fn eq(&self, other: &os_info::Type) -> bool {
    match self {
      Target::Linux => other == &os_info::Type::Linux,
      Target::Darwin => other == &os_info::Type::Macos,
      Target::Win => other == &os_info::Type::Windows,
      Target::Any => true,
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct Block {
  pub name: String,
  pub to: String,
  pub from: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Section {
  pub target: Vec<Target>,
  pub files: Vec<Block>,
  pub from: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
  pub map: Vec<Section>,
}
