//! This module is meant for deserializing the "object model" or "code model"
//! produced by CMake, CTest, and CPack. The terms object model and code model
//! are used fairly interchangeably because CMake is extremely inconsistent.
//! The reason these fields are Deserialize only is because of what I would
//! argue are very terrible "brain dead" decisions. These include Tests having
//! an index into the backtraceGraph, rather than just being IN the
//! backgtraceGraph, CMake itself using multiple "Indexes" (indices, if we're
//! being correct and not "CMake" about it) to put various objects in a more
//! sparse format when they could have just... not done that. It's frustrating
//! overall and a lot of work is going to be done to
//! 1. Clean up the deserialized formats of each program (CMake, CTest, and CPack)
//! into a single structure
//! 2. Never actually handing this raw format to users.
//! 3. Lastly, some work will be needed to make sure that, given an async CMake
//! command, you can simply call `format!("{}", cmake.await?)` on a given
//! instance and this will print the correct set of cargo rustc-link=static=,
//! lines, etc.
//!
//! These details are discussed in other modules. But I wanted to explain the
//! situation here first.

use crate::internal::*;

#[derive(Deserialize, Debug)]
pub struct Version {
  major: u64,
  minor: u64,
}

pub(crate) mod test {
use super::*;

#[derive(Deserialize, Debug)]
struct Node {
  command: u64,
  parent: u64,
  file: u64,
  line: u64,
}

#[derive(Deserialize, Debug)]
struct BacktraceGraph {
  commands: Vec<String>,
  files: Vec<PathBuf>,
  nodes: Vec<Node>,
}

#[derive(Deserialize, Debug)]
pub struct Test {
  // This field type is temporary. Some more "work" is needed to nail things down.
  properties: HashMap<String, OsString>,
  backtrace: u64, // index into the 'nodes' member of the backtraceGraph
  config: String,
  name: String,
}

}

mod package { }
mod build {
}
