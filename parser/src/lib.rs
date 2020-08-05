use dtflib::{Context, DotFile};
use read_file::read_file;
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

  pub fn read_config(&mut self, path: &PathBuf) -> Result<()> {
    match &self.config {
      Some(_) => {}
      None => {
        let config: Config = read_file(path)?;
        self.config = Some(config);
      }
    }

    Ok(())
  }

  pub fn config(&self) -> Option<&Config> {
    match &self.config {
      Some(config) => Some(config),
      None => None,
    }
  }

  pub fn parse(&mut self, path: &PathBuf) -> Result<HashMap<u32, DotFile>> {
    self.read_config(path)?;

    Ok(mapping::map(self.cx, self.config.as_ref().unwrap())?)
  }
}
