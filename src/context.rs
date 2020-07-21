use crate::lib::{client_os, User};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context<'a> {
  pub config_path: &'a PathBuf,
  pub base_dir: &'a PathBuf,
  pub client_os: &'a client_os::Type,
  pub home_dir: &'a PathBuf,
  pub user: &'a User,
}
