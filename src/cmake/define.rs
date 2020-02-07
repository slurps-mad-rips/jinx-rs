use crate::internal::*;

pub enum Define {
  Bool(bool), // ON, OFF, YES, NO, TRUE, FALSE, 1, 0, Y, N, *-NOTFOUND, IGNORE
  Directory(std::path::PathBuf),
  Filepath(std::path::PathBuf),
  Internal(OsString),
  String(OsString),
  Any(String),
}

impl Into<Define> for std::path::PathBuf {
  fn into (self) -> Define {
    use Define::*;
    if self.file_name().is_some() { Filepath(self) } else { Directory(self) }
  }
}

impl Into<Define> for String {
  fn into (self) -> Define {
    use Define::*;
    match self.as_str() {
      "NO" | "OFF" | "N" | "0" | "FALSE" | "IGNORE" => Bool(false),
      "YES" | "ON" | "Y" | "1" | "TRUE" => Bool(true),
      value @ _ if value.ends_with("-NOTFOUND") => Bool(false),
      _ => String(self.into())
    }
  }
}


