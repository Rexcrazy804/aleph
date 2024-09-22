use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

// bin attributes are kinda messy so we needa use this enum
// NOTE I HOPE the other attributes are fine with the normal
// OneOrMany enum, I have no Idea how I am supposed to handle this .w.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Bin<T> {
    One(T),
    Many(Vec<T>),
    TooMany(Vec<Bin<T>>),
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    // REQUIRED properties
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: License,

    // rest are optoinal .w.
    pub bin: Option<Bin<String>>,
    pub depends: Option<OneOrMany<String>>,
    pub env_add_path: Option<OneOrMany<String>>,
    // TODO: this must be a hasmap, write a test for this
    pub env_set: Option<HashMap<String, String>>,
    pub extract_dir: Option<String>,
    pub extract_to: Option<OneOrMany<String>>,
    pub hash: Option<OneOrMany<String>>,
    pub innosetup: Option<bool>,
    pub notes: Option<OneOrMany<String>>,
    pub psmodule: Option<ModuleName>,
    pub url: Option<OneOrMany<String>>,
    //pub architecture: Option<HashMap<String, ArchManifest>>,
    pub architecture: Option<Architecture>,

    #[serde(rename = "##")]
    pub comment: Option<String>,
    // incomplete implementation
    // TODO implement the ones that can be implemeneted :)
    // ignored/unimplemented:
    // autoupdate: AutoUpdate
    // checkver: Regex
    // shortcuts: Vec<(String, String)>
    // suggest: Vec<(String, String)>
    //
    // NOTE high priority
    // installer: Script
    // uninstaller: Script
    // post_install: Vec<Script>
    // post_install: Vec<Script>
    // pre_uninstall: Vec<Script>
    // post_uninstall: Vec<Script>
    // persist: Vec<Directory>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum License {
    License(String),
    Custom(CustomLicense),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CustomLicense {
    pub identifier: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ModuleName {
    pub name: String,
}

// I am hungry as fuck, but I just experienced spiritual awakening from writing this structure
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Architecture {
    #[serde(rename = "64bit")]
    pub x86_65: Option<ArchManifest>,
    #[serde(rename = "32bit")]
    pub x64: Option<ArchManifest>,
    pub arm64: Option<ArchManifest>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArchManifest {
    pub bin: Option<Bin<String>>,
    pub extract_dir: Option<String>,
    pub url: Option<OneOrMany<String>>,
    pub hash: Option<OneOrMany<String>>,
    // unimplemnted
    //uninstaller
    //shortcuts
    //checkver
    //installer
    //pre_install
    //post_install
}
