use crate::lib::client_os;
use serde::Deserialize;

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

impl std::cmp::PartialEq<client_os::Type> for Target {
  fn eq(&self, x: &client_os::Type) -> bool {
    match self {
      Target::Linux => x == &client_os::Type::Linux,
      Target::Darwin => x == &client_os::Type::Darwin,
      Target::Win => x == &client_os::Type::Win,
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
