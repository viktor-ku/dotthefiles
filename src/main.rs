use std::collections::HashMap;
use std::io::Result;
use structopt::StructOpt;

mod app;
use app::App;

mod context;
use context::Context;

mod lib;
use lib::{client_os, config::Config, mapping, read_yaml, DotFile};

mod cmd;

mod cnst;
pub use cnst::CHILD_PARAM;

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().filter(|arg| arg != CHILD_PARAM).collect();
  let app = App::from_iter(args);

  let os_info = os_info::get().os_type();
  let client_os = &client_os::Type::from(&os_info);
  let home_dir = &dirs::home_dir().unwrap();

  let child: bool = match std::env::args().find(|arg| arg == CHILD_PARAM) {
    None => false,
    Some(_) => true,
  };

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
        child,
      };

      if cx.is_main() {
        let config: Config = read_yaml(&config_path)?;
        let dotfiles = mapping::map(&cx, &config).unwrap();

        cmd::link(&cx, &dotfiles)?;
      } else {
        let mut dotfiles_json = String::with_capacity(256);
        std::io::stdin().read_line(&mut dotfiles_json)?;

        let dotfiles: HashMap<u32, DotFile> = serde_json::from_str(&dotfiles_json).unwrap();

        cmd::link(&cx, &dotfiles)?;
      }
    }
  }

  Ok(())
}
