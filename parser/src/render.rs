use std::path::{Component, PathBuf};

#[derive(Debug)]
pub struct RenderState<'a> {
  pub home_dir: &'a PathBuf,
  pub base_dir: &'a PathBuf,
  pub source_dir: &'a str,
}

#[derive(Debug)]
pub struct Render<'a> {
  body: &'a str,
}

impl<'a> Render<'a> {
  pub fn normalize(&self, state: &RenderState) -> PathBuf {
    let mut p = PathBuf::with_capacity(self.body.len() * 2);
    let body = &PathBuf::from(self.body);

    if body.is_absolute() {
      p.push("/");
      p.push(body)
    } else if body.is_relative() {
      p.push(state.base_dir);
      p.push(body);
    } else if body.starts_with("~") {
      p.push(state.home_dir);

      let stripped = body.strip_prefix("~/").unwrap();
      if !stripped.to_path_buf().to_str().unwrap_or("").is_empty() {
        p.push(stripped);
      }
    }

    p
  }

  pub fn render(&self, state: &RenderState) -> PathBuf {
    let mut p = PathBuf::with_capacity(self.body.len() * 2);
    let norm = &self.normalize(state);

    for one in norm.components() {
      match one {
        Component::Normal(val) => match val.to_str().unwrap() {
          "~" => {
            p.push(&state.home_dir);
          }
          "$TARGET" => {
            if !state.source_dir.is_empty() {
              p.push(state.source_dir);
            }
          }
          val => {
            p.push(val);
          }
        },
        Component::RootDir => {
          p.push("/");
        }
        Component::CurDir => {}
        Component::ParentDir => {}
        Component::Prefix(_) => {}
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
