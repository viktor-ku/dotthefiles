use super::{Block, Target};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Section {
  #[serde(default = "Section::default_target")]
  pub target: Vec<Target>,

  pub files: Vec<Block>,

  #[serde(default = "Section::default_from")]
  pub from: String,
}

impl Section {
  fn default_target() -> Vec<Target> {
    vec![Target::default()]
  }

  fn default_from() -> String {
    String::from("files/$TARGET")
  }
}
