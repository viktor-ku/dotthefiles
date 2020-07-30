use super::Section;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub map: Vec<Section>,
}
