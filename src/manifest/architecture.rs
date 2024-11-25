use super::{Binary, Installer, OneOrMany, Script, Shortcuts};
use serde::{Deserialize, Serialize};

// I am hungry as fuck, but I just experienced spiritual awakening from writing this structure
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Architecture {
    #[serde(rename = "64bit")]
    pub x86_64: Option<ArchManifest>,
    #[serde(rename = "32bit")]
    pub x86: Option<ArchManifest>,
    pub arm64: Option<ArchManifest>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArchManifest {
    pub bin: Option<Binary>,
    pub extract_dir: Option<OneOrMany<String>>,
    pub url: Option<OneOrMany<String>>,
    pub hash: Option<OneOrMany<String>>,
    pub shortcuts: Option<Vec<Shortcuts>>,
    pub installer: Option<Installer>,
    pub post_install: Option<Script>,
    pub pre_install: Option<Script>,
    pub uninstaller: Option<Installer>,
}
