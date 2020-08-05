use std::io::Read;
use std::{fs, io, path::PathBuf};

pub fn read_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> io::Result<T> {
  let mut f = fs::File::open(path)?;

  let v: io::Result<T> = match path
    .extension()
    .expect("config has no extension")
    .to_str()
    .unwrap()
  {
    "json" => Ok(serde_json::from_reader(&f)?),
    "yml" | "yaml" => Ok(serde_yaml::from_reader(&f).expect("Could not parse yaml")),
    "toml" => {
      let mut content = String::with_capacity(4096);
      f.read_to_string(&mut content)?;
      let val: T = toml::from_str(&content)?;
      Ok(val)
    }
    _ => Err(io::Error::new(
      io::ErrorKind::Other,
      "Unsupported config extension",
    )),
  };

  v
}
