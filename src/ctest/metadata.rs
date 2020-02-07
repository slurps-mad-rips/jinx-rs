use crate::internal::*;

#[derive(Deserialize, Debug)]
pub struct Version {
  major: u32,
  minor: u32,
}

#[derive(Deserialize, Debug)]
pub struct Node {
  parent: u64,
  command: u64,
  file: u64,
  line: u64,
}

#[derive(Deserialize, Debug)]
pub struct BacktraceGraph {
  commands: Vec<String>,
  files: Vec<PathBuf>,
  nodes: Vec<Node>,
}

#[derive(Deserialize, Debug)]
pub struct Test {
  properties: HashMap<String, String>,
  backtrace: u64,
  config: String,
  name: String,
}

#[derive(Deserialize, Debug)]
pub struct Information {
  version: Version,
  tests: Vec<Test>,
  #[serde(alias="backtraceGraph", flatten)]
  graph: BacktraceGraph,
}
