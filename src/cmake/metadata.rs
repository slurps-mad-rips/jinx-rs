use crate::internal::*;

// TODO: This can be optimized and corrected for proper behavior
#[derive(Deserialize, Debug)]
pub struct Entry {
  properties: HashMap<String, OsString>,
  value: OsString,
  name: String,
  #[serde(alias="type")]
  kind: String,
}

#[derive(Deserialize, Debug)]
pub struct Cache {
  entries: Vec<Entry>,
  version: Version,
}

#[derive(Deserialize, Debug)]
pub struct Input {
  path: PathBuf,
  #[serde(rename="isGenerated")]
  generated: Option<bool>,
  #[serde(rename="isExternal")]
  external: Option<bool>,
  #[serde(rename="isCMake")]
  builtin: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Files {
  inputs: Vec<Input>,
}
