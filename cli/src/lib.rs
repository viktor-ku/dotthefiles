use dtflib::client_os;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dtf")]
pub enum Cli {
  #[structopt(name = "ln")]
  Link {
    #[structopt(name = "config-path", parse(from_os_str))]
    config: PathBuf,

    #[structopt(
      short,
      long,
      help = "overrides destination file if one exists when making a hard link"
    )]
    force: bool,

    #[structopt(
      long,
      parse(from_str = client_os::Type::from),
    )]
    os: Option<client_os::Type>,
  },

  #[structopt(name = "ls")]
  List {
    #[structopt(name = "config-path", parse(from_os_str))]
    config: PathBuf,

    #[structopt(
      long,
      parse(from_str = client_os::Type::from),
    )]
    os: Option<client_os::Type>,
  },

  Show {
    #[structopt(name = "config-path", parse(from_os_str))]
    config: PathBuf,
  },
}

pub struct App;

impl App {
  pub fn with_args(args: &Vec<String>) -> Cli {
    Cli::from_iter(args)
  }
}

mod cmd;
pub use cmd::*;

mod sudo;
use sudo::sudo;

mod report;
use report::Report;

mod hard_link;
