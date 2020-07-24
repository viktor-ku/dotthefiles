use dtflib::{Context, DotFile};
use read_file::read_yaml;
use std::collections::HashMap;
use std::io::Result;
use std::path::PathBuf;

mod read_file;

mod config;
use config::Config;

mod mapping;

mod render;
use render::{Render, RenderState};

pub struct Parser<'a> {
  config: Option<Config>,
  json: Option<&'a str>,
  cx: &'a Context<'a>,
}

impl<'a> Parser<'a> {
  pub fn with(cx: &'a Context) -> Self {
    Self {
      config: None,
      json: None,
      cx,
    }
  }

  pub fn from_path(&mut self, path: &PathBuf) -> Result<&Self> {
    let config: Config = read_yaml(path)?;
    self.config = Some(config);
    Ok(self)
  }

  pub fn from_json(&mut self, json: &'a str) -> &Self {
    self.json = Some(json);
    self
  }

  pub fn parse(&self) -> Result<HashMap<u32, DotFile>> {
    if self.json.is_some() {
      Ok(serde_json::from_str(self.json.unwrap())?)
    } else {
      Ok(mapping::map(self.cx, self.config.as_ref().unwrap())?)
    }
  }
}
