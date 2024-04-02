use anyhow::Error;

pub fn hello() -> Result<&'static str, Error> {
  Ok("hello world")
}
