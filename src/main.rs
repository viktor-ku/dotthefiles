use std::io;
use structopt::StructOpt;

mod cli;
use cli::Cli;

mod lib;
use lib::config;
use lib::mapping;
use lib::read_yaml;

#[tokio::main]
async fn main() -> io::Result<()> {
  let cli = Cli::from_args();
  let config_path = cli.config.canonicalize()?;

  if config_path.is_dir() {
    panic!("config should point to the dotthefiles.json or similar mappings config");
  }

  let mut base_dir = config_path.clone();
  base_dir.set_file_name("");

  let config: config::Config = read_yaml(&config_path)?;

  let mapping = mapping::Mapping {
    base_dir: &base_dir,
    os_type: &os_info::get().os_type(),
    home_dir: &dirs::home_dir().unwrap(),
  };

  let files = mapping.map(&config)?;
  println!("{:#?}", files);

  Ok(())
}
