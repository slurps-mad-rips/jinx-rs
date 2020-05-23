//use crate::internal::*;
//
///// Represents one of the several types of values a user can define on the
///// command line
//pub enum Define {
//  /// Path to a directory on disk. `cmake-gui` offers a file dialog
//  Directory(PathBuf),
//  /// Path to a file on disk. `cmake-gui` offers a file dialog
//  Filepath(PathBuf),
//  /// Does not show under `cmake-gui`
//  Internal(OsString),
//  /// A line of text. `cmake-gui` offers a text field.
//  /// If the `STRINGS` cache entry property is set, a drop-down selection box
//  /// is shown.
//  String(OsString),
//  /// This becomes the "untyped" entry.
//  Any(String),
//  /// ON/OFF value. `cmake-gui` offers a file dialog.
//  Bool(bool),
//}
//
///// If the string is equal to `ON`, `OFF`, `YES`, `NO`, `TRUE`, `FALSE`, `1`,
///// `0`, `Y`, `N`, or `IGNORE`, Jinx will convert it to a Define::Bool
//impl Into<Define> for String {
//  fn into (self) -> Define {
//    use Define::*;
//    match self.as_str() {
//      "IGNORE" | "FALSE" | "OFF" | "NO" | "N" | "0" => Bool(false),
//      "TRUE" | "YES" | "ON" | "Y" | "1" => Bool(true),
//      value @ _ if value.ends_with("-NOTFOUND") => Bool(false),
//      _ => String(self.into()),
//    }
//  }
//}
//
///// If the `PathBuf` has a filename, a Filepath will be returned.
//impl Into<Define> for PathBuf {
//  fn into (self) -> Define {
//    use Define::*;
//    if self.file_name().is_some() { Filepath(self) } else { Directory(self) }
//  }
//}
