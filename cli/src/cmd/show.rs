use std::io::Result;

pub fn show(config: &str) -> Result<()> {
  println!("{}", config);
  Ok(())
}
