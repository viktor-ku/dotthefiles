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
    println!("|> {}: {}", dotfile.name.bold(), err.message.red());
    println!("   - Error occured while trying to {}", err.stage);
    match err.stage {
      dotfile::ErrorStage::HardLink => {
        println!(
          "     from: {}",
          dotfile
            .src_file_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .dimmed()
        );
        println!(
          "     to  : {}",
          dotfile
            .dst_file_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .dimmed()
        );
      }
      dotfile::ErrorStage::RemoveFile => {
        println!(
          "     {}",
          dotfile
            .dst_file_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .dimmed()
        );
      }
    }
  }
}
