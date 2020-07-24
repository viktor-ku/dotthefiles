use cli::{self, App, Cli};
use dtflib::{client_os, Context};
use parser::Parser;
use std::io::Result;

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
    Cli::Link { config } => {
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

      let mut parser = Parser::with(&cx);

      if cx.is_main() {
        let dotfiles = parser.from_path(&config_path)?.parse()?;

        cli::link(&cx, &dotfiles)?;
      } else {
        let mut dotfiles_json = String::with_capacity(256);
        std::io::stdin().read_line(&mut dotfiles_json)?;

        let dotfiles = parser.from_json(&dotfiles_json).parse()?;

        cli::link(&cx, &dotfiles)?;
      }
    }
  }

  Ok(())
}
