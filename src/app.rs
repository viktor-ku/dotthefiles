use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dtf")]
pub enum App {
  #[structopt(name = "ln")]
  Link {
    #[structopt(name = "config-path", parse(from_os_str))]
    config: PathBuf,
  },
}
