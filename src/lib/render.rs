use std::path::PathBuf;

#[derive(Debug)]
pub struct Render<'a> {
  body: &'a str,
}

impl<'a> Render<'a> {
  pub fn render(&self, home_dir: &PathBuf) -> PathBuf {
    let mut p = PathBuf::with_capacity(self.body.len() * 2);
    let body = &PathBuf::from(self.body);

    if body.starts_with("~/") {
      p.push(&home_dir);

      let stripped = body.strip_prefix("~/").unwrap();

      if !stripped.to_path_buf().to_str().unwrap().is_empty() {
        p.push(body.strip_prefix("~/").unwrap());
      }
    }

    p
  }
}

impl<'a> std::convert::From<&'a String> for Render<'a> {
  fn from(val: &'a String) -> Self {
    Self { body: val }
  }
}
