use crate::lib::dotfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
  pub dotfile: u32,
  pub error: Option<dotfile::Error>,
}
