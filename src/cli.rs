use async_std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dotthefiles")]
pub struct Cli {
  #[structopt(name = "config", parse(from_os_str))]
  pub config: PathBuf,
}
