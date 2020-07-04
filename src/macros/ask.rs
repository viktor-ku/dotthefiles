#[macro_export]
macro_rules! ask {
  ($q:ident, $text:expr) => {
    match $q.ask($text) {
      None => return Ok(()),
      Some(val) if !val => return Ok(()),
      Some(_) => {}
    };
  };
}
