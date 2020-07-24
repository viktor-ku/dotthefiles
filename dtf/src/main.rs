use std::collections::HashMap;
use std::io::Result;

use cli::{self, App, Cli};
use dtflib::{client_os, Context};

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

      if cx.is_main() {
        let config: parser::config::Config = parser::read_yaml(&config_path)?;
        let dotfiles = parser::mapping::map(&cx, &config).unwrap();

        cli::link(&cx, &dotfiles)?;
      } else {
        let mut dotfiles_json = String::with_capacity(256);
        std::io::stdin().read_line(&mut dotfiles_json)?;

        let dotfiles: HashMap<u32, dtflib::DotFile> = serde_json::from_str(&dotfiles_json).unwrap();

        cli::link(&cx, &dotfiles)?;
      }
    }
  }

  Ok(())
}
