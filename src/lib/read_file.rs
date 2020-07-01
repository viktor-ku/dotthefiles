use async_std::{
  fs, io,
  io::{BufReader, ReadExt, Result},
  path::PathBuf,
};

pub async fn read_file(path: &PathBuf) -> Result<String> {
  let file = fs::File::open(path).await?;
  let mut buf_reader = BufReader::new(file);
  let mut package_json_str = String::new();
  buf_reader.read_to_string(&mut package_json_str).await?;

  Ok(package_json_str)
}

pub async fn read_yaml<T: serde::de::DeserializeOwned>(path: &PathBuf) -> io::Result<T> {
  let data = read_file(path).await?;
  let v: T = serde_yaml::from_str(&data).expect("Could not parse yaml");

  Ok(v)
}
