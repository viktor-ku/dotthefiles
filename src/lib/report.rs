use crate::lib::dotfile::DotFile;

#[derive(Debug)]
pub enum Op {
  Created,
  Replaced,
  Skipped,
}

#[derive(Debug)]
pub struct Report<'a> {
  pub op: &'a Op,
  pub file: &'a DotFile,
}

impl<'a> Report<'a> {
  pub fn new(op: &'a Op, file: &'a DotFile) -> Self {
    Self { op, file }
  }
}
