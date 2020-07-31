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
  cx: &'a Context<'a>,
  config: Option<Config>,
}

impl<'a> Parser<'a> {
  pub fn with(cx: &'a Context) -> Self {
    Self { cx, config: None }
  }

  pub fn parse(&mut self, path: &PathBuf) -> Result<HashMap<u32, DotFile>> {
    let config: Config = read_yaml(path)?;
    self.config = Some(config);

    Ok(mapping::map(self.cx, self.config.as_ref().unwrap())?)
  }
}
