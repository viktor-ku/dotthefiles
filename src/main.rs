use std::io::Result;
use structopt::StructOpt;

mod app;
use app::App;

mod macros;

fn main() -> Result<()> {
  let app = App::from_args();

  Ok(())
}
