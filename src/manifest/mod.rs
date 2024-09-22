pub mod architecture;
pub mod installer;
pub mod license;
pub mod shortcuts;

use architecture::Architecture;
use installer::{Installer, Script};
use license::License;
use serde::{Deserialize, Serialize};
use shortcuts::Shortcuts;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

// bin attributes are kinda messy so we needa use this enum
// same for persist
// TODO: come up with a better name for this
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum WayTooMany<T> {
    One(T),
    Many(Vec<T>),
    TooMany(Vec<WayTooMany<T>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    // REQUIRED properties
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: License,

    // rest are optoinal .w.
    pub bin: Option<WayTooMany<String>>,
    pub depends: Option<OneOrMany<String>>,
    pub env_add_path: Option<OneOrMany<String>>,

    pub extract_dir: Option<OneOrMany<String>>,
    pub extract_to: Option<OneOrMany<String>>,
    pub persist: Option<WayTooMany<String>>,
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
