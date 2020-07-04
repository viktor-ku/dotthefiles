use colored::Colorize;
use rustyline::Editor;

#[derive(Debug)]
pub struct Answer<'a> {
  val: &'a str,
}

impl<'a> Answer<'a> {
  pub fn new(val: &'a str) -> Self {
    Self { val }
  }
}

impl std::convert::From<Answer<'_>> for bool {
  fn from(answer: Answer) -> bool {
    let val = answer.val.trim();

    match val {
      "Y" | "y" | "Yes" | "yes" => true,
      _ => false,
    }
  }
}

#[derive(Debug)]
pub struct Question {
  rl: Editor<()>,
}

impl Question {
  pub fn new() -> Self {
    Self {
      rl: Editor::<()>::new(),
    }
  }

  pub fn ask(&mut self, question: &str) -> Option<bool> {
    let readline = &self
      .rl
      .readline(&format!("|> {} (y/N): ", question.green().bold()));

    match readline {
      Ok(line) => Some(Answer::new(&line).into()),
      Err(_) => None,
    }
  }
}
