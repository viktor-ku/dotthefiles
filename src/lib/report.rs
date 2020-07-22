use crate::lib::{dotfile, DotFile};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
  pub dotfile_id: u32,
  pub error: Option<dotfile::Error>,
}

impl Report {
  pub fn is_ok(&self) -> bool {
    self.error.is_none()
  }

  pub fn print(dotfile: &DotFile, err: &dotfile::Error) {
    println!("- {}", dotfile.name);
    println!("  {}: {}", "Err".red(), err);
    match err.source {
      dotfile::ErrorSource::Src => {
        println!(
          "       {}",
          dotfile.src.as_os_str().to_str().unwrap().dimmed()
        );
      }
      dotfile::ErrorSource::Dst => {
        println!(
          "       {}",
          dotfile.dst.as_os_str().to_str().unwrap().dimmed()
        );
      }
    }
  }
}
