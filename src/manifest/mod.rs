pub mod architecture;
pub mod bin;
pub mod installer;
pub mod license;
pub mod persist;
pub mod shortcuts;

use architecture::{ArchManifest, Architecture};
use bin::Binary;
use installer::{Installer, Script};
use license::License;
use persist::Persist;
use serde::{Deserialize, Serialize};
use shortcuts::Shortcuts;
use std::collections::{HashMap, VecDeque};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(VecDeque<T>),
}

impl<T: Clone> Iterator for OneOrMany<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let OneOrMany::One(lonely_data) = self {
            *self = OneOrMany::Many(VecDeque::from([lonely_data.clone()]));
        }

        let OneOrMany::Many(vector) = self else {
            unreachable!();
        };

        vector.pop_front()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    // REQUIRED properties
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: License,

    // rest are optoinal .w.
    pub bin: Option<Binary>,
    pub depends: Option<OneOrMany<String>>,
    pub env_add_path: Option<OneOrMany<String>>,

    pub extract_dir: Option<OneOrMany<String>>,
    pub extract_to: Option<OneOrMany<String>>,
    pub persist: Option<OneOrMany<Persist>>,
    pub hash: Option<OneOrMany<String>>,
    pub innosetup: Option<bool>,
    pub notes: Option<OneOrMany<String>>,
    pub psmodule: Option<ModuleName>,
    pub url: Option<OneOrMany<String>>,
    pub architecture: Option<Architecture>,

    // TODO: write tests for these
    pub suggest: Option<HashMap<String, OneOrMany<String>>>,
    pub env_set: Option<HashMap<String, String>>,
    pub shortcuts: Option<Vec<Shortcuts>>,
    pub installer: Option<Installer>,
    pub post_install: Option<Script>,
    pub pre_install: Option<Script>,
    pub uninstaller: Option<Installer>,
    pub post_uinstall: Option<Script>,
    pub pre_uinstall: Option<Script>,

    #[serde(rename = "##")]
    pub comment: Option<String>,
    // NOTE
    // these are meant for the scoop repo to auto update
    // autoupdate: AutoUpdate
    // checkver: Regex
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ModuleName {
    pub name: String,
}

impl Manifest {
    pub fn parse(str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(str)
    }

    /// this function is a simple interface to extract the url from the manifest
    /// all functional manifests WILL HAVE atleast one valid url in
    /// ```Manifest.arch or Manifest.architecture.*.url```
    /// # Panics
    /// this function can panic if no url: or Architecture.<arch>.url is found
    #[must_use]
    pub fn get_url(&self) -> OneOrMany<String> {
        if let Some(url) = &self.url {
            return url.clone();
        };

        let Some(arch) = &self.architecture else {
            panic!("No URL FOUND")
        };

        let arch = arch.clone();
        let os_arch = std::env::consts::ARCH;
        if os_arch == "x86" {
            let Some(ArchManifest { url, .. }) = arch.x86 else {
                panic!("No URL FOUND")
            };
            url.unwrap()
        } else if os_arch == "x86_64" {
            let Some(ArchManifest { url, .. }) = arch.x86_64 else {
                panic!("No URL FOUND")
            };
            url.unwrap()
        } else if os_arch == "aarch64" {
            let Some(ArchManifest { url, .. }) = arch.arm64 else {
                panic!("No URL FOUND")
            };
            url.unwrap()
        } else {
            panic!("NO URL");
        }
    }
}
