use crate::internal::*;
use self::define::Define;

mod metadata;
mod define;

pub async fn configure (_cfg: Configuration) -> Result<()> {
  Ok(())
}

pub struct Configuration {
  generator: String,
  defines: Vec<(OsString, Define)>,
  env: Vec<(OsString, OsString)>,
}

impl Configuration {
  pub fn env (&mut self, key: impl AsRef<OsString>, value: impl AsRef<OsString>) -> &mut Self {
    self.env.push((key.as_ref().into(), value.as_ref().into()));
    self
  }

  pub fn define (&mut self, var: impl AsRef<OsString>, value: impl Into<Define>) -> &mut Self {
    self.defines.push((var.as_ref().into(), value.into()));
    self
  }

  //pub fn env_path (&mut self, key: impl AsRef<OsString>, value: impl Iterator<Item=impl AsRef<OsString>>) -> &mut Self {
  //  self
  //}
}
