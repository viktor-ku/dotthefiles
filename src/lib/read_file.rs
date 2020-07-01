use std::{fs, io, path::PathBuf};

pub fn read_yaml<T: serde::de::DeserializeOwned>(path: &PathBuf) -> io::Result<T> {
  let f = fs::File::open(path)?;
  let v: T = serde_yaml::from_reader(&f).expect("Could not parse yaml");
  Ok(v)
}
