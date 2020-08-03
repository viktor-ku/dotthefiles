use std::{fs, io, path::PathBuf};

pub fn read_yaml<T: serde::de::DeserializeOwned>(path: &PathBuf) -> io::Result<T> {
  let f = fs::File::open(path)?;

  let v: io::Result<T> = match path
    .extension()
    .expect("config has no extension")
    .to_str()
    .unwrap()
  {
    "json" => Ok(serde_json::from_reader(&f)?),
    "yml" | "yaml" => Ok(serde_yaml::from_reader(&f).expect("Could not parse yaml")),
    _ => Err(io::Error::new(
      io::ErrorKind::Other,
      "Unsupported config extension",
    )),
  };

  v
}
