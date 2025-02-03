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
    /// # Errors
    /// - invalid json input would result in a ``serde_json::Error``.
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
            // no other places to look for url if url tag has nothing
            // and architecture tag is empty
            panic!("No url FOUND")
        };

        let arch = arch.clone();
        let os_arch = std::env::consts::ARCH;
        match os_arch {
            "x86" => {
                if let Some(ArchManifest { url: Some(url), .. }) = arch.x86 {
                    return url;
                }
            }
            "x86_64" => {
                if let Some(ArchManifest { url: Some(url), .. }) = arch.x86_64 {
                    return url;
                }
            }
            "aarch64" => {
                if let Some(ArchManifest { url: Some(url), .. }) = arch.arm64 {
                    return url;
                }
            }
            _ => {
                panic!("Un supported architecture")
            }
        };

        panic!("No url found");
    }

    /// a function to retreive a valid bin attribute found withing the manifest
    #[must_use]
    pub fn get_bin(&self) -> Option<Binary> {
        if self.bin.is_some() {
            return self.bin.clone();
        };

        let Some(arch) = &self.architecture else {
            // no other places to look for bin if bin tag has nothing
            // and architecture tag is empty
            return None;
        };

        let arch = arch.clone();
        let os_arch = std::env::consts::ARCH;
        match os_arch {
            "x86" => {
                if let Some(ArchManifest { bin, .. }) = arch.x86 {
                    return bin;
                }
            }
            "x86_64" => {
                if let Some(ArchManifest { bin, .. }) = arch.x86_64 {
                    return bin;
                }
            }
            "aarch64" => {
                if let Some(ArchManifest { bin, .. }) = arch.arm64 {
                    return bin;
                }
            }
            _ => {
                eprintln!("Un supported architecture");
            }
        };

        None
    }
}
