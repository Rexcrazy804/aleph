use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    // REQUIRED properties
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: String,

    // rest are optoinal .w.
    // NOTE that most of the Vec<T> are likely to fail
    // will require the followign implementation
    // https://serde.rs/string-or-struct.html
    pub bin: Option<OneOrMany<String>>,
    pub depends: Option<OneOrMany<String>>,
    pub env_add_path: Option<String>,
    // I am not sure if this works the way I hope it works
    // TODO: Test whether this works in accordance to the manifest
    pub env_set: Option<OneOrMany<(String, String)>>,
    pub extract_dir: Option<String>,
    pub extract_to: Option<String>,
    pub hash: Option<OneOrMany<String>>,
    pub innosetup: Option<bool>,
    pub notes: Option<OneOrMany<String>>,
    pub psmodule: Option<ModuleName>,
    pub url: Option<OneOrMany<String>>,

    // incomplete implementation
    // in the scoop manifest comments start with "##" we'll have to make a custom deserializer for
    // this .w.
    comment: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct ModuleName {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>)
}
