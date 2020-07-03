use std::io;
use structopt::StructOpt;

mod cli;
use cli::Cli;

mod lib;
use lib::client_os;
use lib::config;
use lib::mapping;
use lib::read_yaml;

fn main() -> io::Result<()> {
  let cli = Cli::from_args();
  let config_path = cli.config.canonicalize()?;

  if config_path.is_dir() {
    panic!("config should point to the dotthefiles.json or similar mappings config");
  }

  let mut base_dir = config_path.clone();
  base_dir.set_file_name("");

  let config: config::Config = read_yaml(&config_path)?;

  let client_os_info = &os_info::get().os_type();

  let mapping = mapping::Mapping {
    base_dir: &base_dir,
    home_dir: &dirs::home_dir().unwrap(),
    client_os: &client_os::Type::from(client_os_info),
  };

  let files = mapping.map(&config)?;

  if cli.list {
    for file in &files {
      println!("{:?};{:?}", file.from(), file.to());
    }

    return Ok(());
  }

  Ok(())
}
