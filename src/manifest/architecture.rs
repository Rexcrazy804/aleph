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

impl Architecture {
    #[must_use]
    pub fn get_arch_manifest(&self) -> Option<&ArchManifest> {
        let os_arch = std::env::consts::ARCH;
        match os_arch {
            "x86" => self.x86.as_ref(),
            "x86_64" => self.x86_64.as_ref(),
            "a&self64" => self.arm64.as_ref(),
            _ => None,
        }
    }
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
