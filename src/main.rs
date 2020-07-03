use comfy_table::Table;
use structopt::StructOpt;

mod cli;
use cli::Cli;

mod lib;
use lib::{client_os, mapping, op, read_yaml};

fn main() -> std::io::Result<()> {
  let cli = &Cli::from_args();
  let config_path = &cli.config.canonicalize()?;

  if config_path.is_dir() {
    panic!("config should point to the dotthefiles.json or similar mappings config");
  }

  let mut base_dir = config_path.clone();
  base_dir.set_file_name("");

  let config = &read_yaml(&config_path)?;

  let client_os_info = &os_info::get().os_type();

  let mapping = &mapping::Mapping {
    base_dir: &base_dir,
    home_dir: &dirs::home_dir().unwrap(),
    client_os: &client_os::Type::from(client_os_info),
  };

  let files = &mapping.map(&config)?;

  if cli.list {
    for file in files {
      println!("{:?},{:?}", file.from(), file.to());
    }
    return Ok(());
  }

  println!("What exactly am I about to do:");

  let mut table = Table::new();
  table.set_header(vec!["Action", "From", "To"]);

  for file in files {
    let from = &file.from();
    let from_exists = from.exists();

    let to = &file.to();
    let to_exists = to.exists();

    let op = {
      if !from_exists {
        op::Op::Skip(op::Reason::NoSource)
      } else if to_exists {
        if cli.skip {
          op::Op::Skip(op::Reason::DestExists)
        } else {
          op::Op::Replace
        }
      } else {
        op::Op::Create
      }
    };

    table.add_row(vec![
      &op.into(),
      &format!("{:?}", from),
      &format!("{:?}", to),
    ]);
  }

  println!("{}", table);

  Ok(())
}
