use crate::client_os;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context<'a> {
  pub config_path: &'a PathBuf,
  pub base_dir: &'a PathBuf,
  pub client_os: &'a client_os::Type,
  pub home_dir: &'a PathBuf,

  /// whether the current process is a child (spawned) or main
  pub child: bool,
}

impl<'a> Context<'a> {
  pub fn is_main(&self) -> bool {
    !self.child
  }
}
