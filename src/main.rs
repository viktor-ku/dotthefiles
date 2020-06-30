use async_std::io;
use structopt::StructOpt;

mod cli;
use cli::Cli;

mod lib;
use lib::config;
use lib::mapping;
use lib::read_file;

#[tokio::main]
async fn main() -> io::Result<()> {
  let cli = Cli::from_args();
  let config_path = cli.config.canonicalize().await?;

  if config_path.is_dir().await {
    panic!("config should point to the dotthefiles.json or similar mappings config");
  }

  let mut base_dir = config_path.clone();
  base_dir.set_file_name("");

  let config_yaml_str = read_file(&config_path).await?;
  let config: config::Config =
    serde_yaml::from_str(&config_yaml_str).expect("Could not parse yaml");

  let mapping = mapping::Mapping {
    base_dir: &base_dir,
    os_type: &os_info::get().os_type(),
  };

  let files = mapping.map(&config).await?;
  println!("{:#?}", files);

  Ok(())
}
