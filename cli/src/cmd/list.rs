use colored::Colorize;
use dtflib::{Context, DotFile};
use std::collections::HashMap;
use std::io::Result;

pub fn list(_cx: &Context, dotfiles: &HashMap<u32, DotFile>) -> Result<()> {
  for (_, dotfile) in dotfiles {
    println!("{}", dotfile.name.bold());
    println!(
      "\tfrom {}",
      dotfile.src_file_path().to_str().unwrap().dimmed()
    );
    println!(
      "\t  to {}",
      dotfile.dst_file_path().to_str().unwrap().dimmed()
    );
  }

  Ok(())
}
