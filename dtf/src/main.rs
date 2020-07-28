use cli::{self, App, Cli};
use dtflib::{client_os, Context};
use parser::Parser;
use std::io::Result;

mod validate_config;
use validate_config::validate_config;

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args()
    .filter(|arg| arg != dtflib::CHILD_PARAM)
    .collect();
  let app = App::with_args(&args);

  let os_info = os_info::get().os_type();
  let client_os = &client_os::Type::from(&os_info);
  let home_dir = &dirs::home_dir().unwrap();

  let child: bool = match std::env::args().find(|arg| arg == dtflib::CHILD_PARAM) {
    None => false,
    Some(_) => true,
  };

  match &app {
    Cli::Link { config, force } => {
      let (config_path, base_dir) = &validate_config(&config);

      let cx = Context {
        config_path,
        base_dir,
        client_os,
        home_dir,
        child,
      };

      let mut parser = Parser::with(&cx);

      if cx.is_main() {
        let dotfiles = parser.from_path(&config_path)?.parse()?;

        cli::link(&cx, &dotfiles, *force)?;
      } else {
        let mut dotfiles_json = String::with_capacity(256);
        std::io::stdin().read_line(&mut dotfiles_json)?;

        let dotfiles = parser.from_json(&dotfiles_json).parse()?;

        cli::link(&cx, &dotfiles, *force)?;
      }
    }
    Cli::List { config } => {
      let (config_path, base_dir) = &validate_config(&config);

      let cx = Context {
        config_path,
        base_dir,
        client_os,
        home_dir,
        child,
      };

      let mut parser = Parser::with(&cx);
      let dotfiles = parser.from_path(&config_path)?.parse()?;

      cli::list(&cx, &dotfiles)?;
    }
  }

  Ok(())
}
