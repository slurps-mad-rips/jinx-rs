use crate::prelude::*;
use std::path::PathBuf;

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
  archive_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct ComponentGroup;

#[derive(Serialize, Deserialize, Debug)]
struct InstallType;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
  #[serde(alias="subDirectory")] subdirectory: PathBuf,
  #[serde(alias="projectName")] name: String,
  directory: PathBuf,
  component: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Description {
  #[serde(alias="packageDescriptionSummary")] summary: Option<String>,
  #[serde(alias="packageDescriptionFile")] file: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
  #[serde(alias="packageVersion")] version: Option<String>,
  #[serde(alias="packageName")] name: Option<String>,
  #[serde(alias="buildConfig")] configuration: Option<String>,
  #[serde(flatten)] description: Description,
  #[serde(alias="defaultDirectoryPermissions")]
  permissions: Option<Vec<String>>,
  #[serde(alias="packagingInstallPrefix")]
  prefix: PathBuf,
  #[serde(alias="errorOnAbsoluteInstallDestination")]
  error_on_absolute_install_destination: bool,
  #[serde(alias="warnOnAbsoluteInstallDestination")]
  warn_on_absolute_install_destination: bool,
  #[serde(alias="setDestDir")]
  destination: bool,
  #[serde(alias="stripFiles")]
  strip_files: bool,
}
