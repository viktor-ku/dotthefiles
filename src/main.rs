use std::io::Result;
use structopt::StructOpt;

mod app;
use app::App;

mod context;
use context::Context;

mod lib;
use lib::{client_os, config::Config, mapping, read_yaml, DotFile, User};

mod macros;

mod cmd;

fn main() -> Result<()> {
  let app = App::from_args();

  let os_info = os_info::get().os_type();
  let client_os = &client_os::Type::from(&os_info);
  let home_dir = &dirs::home_dir().unwrap();
  let user = &User::new();

  match &app {
    App::Link {
      config,
      dotfiles,
      child,
    } => {
      let config_path = &config.canonicalize()?;
      let ref mut base_dir = config_path.clone();
      base_dir.pop();

      let cx = Context {
        config_path,
        base_dir,
        client_os,
        home_dir,
        user,
        child,
      };

      if let Some(dotfiles_string) = dotfiles {
        let dotfiles: Vec<DotFile> = serde_json::from_str(&dotfiles_string).unwrap();

        cmd::link(&cx, &dotfiles)?;
      } else {
        let config: Config = read_yaml(&config_path)?;
        let dotfiles = mapping::map(&cx, &config).unwrap();

        cmd::link(&cx, &dotfiles)?;
      }
    }
  }

  Ok(())
}
