use comfy_table::Table;
use std::fs;
use structopt::StructOpt;

mod cli;
use cli::Cli;

mod lib;
use lib::{
  client_os, mapping, op, read_yaml,
  report::{self, Report},
  DotFile, Question,
};

mod macros;

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

  let ops: Vec<(op::Op, &DotFile)> = files
    .iter()
    .map(|file| {
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

      (op, file)
    })
    .collect();

  println!("What exactly am I about to do:");

  let mut table = Table::new();
  table.set_header(vec!["Action", "From", "To"]);

  for (op, file) in &ops {
    table.add_row(vec![
      String::from(op),
      format!("{:?}", file.from),
      format!("{:?}", file.to),
    ]);
  }

  println!("{}", table);

  let mut q = Question::new();

  ask!(q, "Would you like to proceed?");

  let reports: Vec<Report> = ops
    .iter()
    .map(|(op, file)| match op {
      op::Op::Replace => {
        let from = &file.from();
        let to = &file.to();

        fs::remove_file(to).unwrap();
        fs::hard_link(from, to).unwrap();

        Report::new(&report::Op::Replaced, file)
      }
      op::Op::Create => {
        let from = &file.from();
        let to = &file.to();

        fs::hard_link(from, to).unwrap();

        Report::new(&report::Op::Created, file)
      }
      _ => Report::new(&report::Op::Skipped, file),
    })
    .collect();

  let mut table = Table::new();
  table.set_header(vec!["Action happened", "File"]);

  for report in &reports {
    table.add_row(vec![
      format!("{:?}", report.op),
      format!("{:?}", report.file.to()),
    ]);
  }

  println!("...\nWhat actually happened:");
  println!("{}", table);

  Ok(())
}
