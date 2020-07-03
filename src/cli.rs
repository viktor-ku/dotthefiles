use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dotthefiles")]
pub struct Cli {
  #[structopt(name = "config", parse(from_os_str))]
  pub config: PathBuf,

  #[structopt(
    short = "l",
    long = "--list",
    about = "prints to stdout all files it would link if it was an interactive mode without actually linking anything"
  )]
  pub list: bool,
}
