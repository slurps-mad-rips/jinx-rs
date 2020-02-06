use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Component {
  dependencies: Vec<String>,
  description: String,
  group: Option<String>,
  name: String,
  #[serde(alias="installationTypes")]
  install_types: Vec<String>,
  #[serde(alias="isDownloaded")]
  downloaded: bool,
  #[serde(alias="isDisabledByDefault")]
  disabled: bool,
  #[serde(alias="isRequired")]
  required: bool,
  #[serde(alias="isHidden")]
  hidden: bool,
  #[serde(alias="displayName")]
  display_name: String,
  #[serde(alias="archiveFile")]
  archive_file: Path,
}

#[derive(Serialize, Deserialize, Debug)]
struct ComponentGroup;

#[derive(Serialize, Deserialize, Debug)]
struct InstallType;

#[derive(Serialize, Deserialize, Debug)]
struct Project;
