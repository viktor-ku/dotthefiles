use colored::Colorize;
use std::io::{Error, Result};
use std::path::PathBuf;

fn _validate_config(config: &PathBuf) -> Result<(PathBuf, PathBuf)> {
  let config_path = config.canonicalize()?;

  if config_path.is_dir() {
    return Err(Error::from_raw_os_error(libc::EISDIR));
  }

  let mut base_dir: PathBuf = config_path.clone();
  base_dir.pop();
  Ok((config_path, base_dir))
}

pub fn validate_config(config: &PathBuf) -> (PathBuf, PathBuf) {
  match _validate_config(&config) {
    Ok(res) => res,
    Err(e) => {
      println!("{}", "error:".red());
      match (e.kind(), e.raw_os_error().unwrap_or(-1)) {
        (std::io::ErrorKind::NotFound, _) => {
          println!("\tissue with specified `config-path`: config was not found");
          println!("\t{}", config.to_str().unwrap_or("").dimmed());
        }
        (std::io::ErrorKind::Other, libc::EISDIR) => {
          println!("\tissue with specified `config-path`: config is pointing to a directory");
          println!("\t{}", config.to_str().unwrap_or("").dimmed());
        }
        _ => {
          println!("{}", e.to_string());
          println!("\t{}", config.to_str().unwrap_or("").dimmed());
        }
      }
      std::process::exit(1);
    }
  }
}
