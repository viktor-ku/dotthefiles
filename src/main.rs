use std::io::Result;
use structopt::StructOpt;

mod app;
use app::App;

mod context;
use context::Context;

mod lib;
use lib::{client_os, User};

mod macros;

fn main() -> Result<()> {
  let app = App::from_args();

  let os_info = os_info::get().os_type();
  let client_os = &client_os::Type::from(&os_info);
  let home_dir = &dirs::home_dir().unwrap();
  let user = &User::new();

  match &app {
    App::Link { config } => {
      let config_path = &config.canonicalize()?;
      let ref mut base_dir = config_path.clone();
      base_dir.pop();

      let cx = Context {
        config_path,
        base_dir,
        client_os,
        home_dir,
        user,
      };
    }
  }

  Ok(())
}
