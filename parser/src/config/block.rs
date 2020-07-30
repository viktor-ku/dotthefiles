use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
  pub name: String,
  pub to: String,
}
