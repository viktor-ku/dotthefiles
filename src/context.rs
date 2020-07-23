use crate::lib::client_os;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context<'a> {
  pub config_path: &'a PathBuf,
  pub base_dir: &'a PathBuf,
  pub client_os: &'a client_os::Type,
  pub home_dir: &'a PathBuf,

  /// if main process is running then it should be 0
  pub child: &'a u8,
}

impl<'a> Context<'a> {
  pub fn is_main(&self) -> bool {
    *self.child == 0
  }
}
