use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    // REQUIRED properties
    version: String,
    description: String,
    homepage: String,
    license: String,

    // rest are optoinal .w.
    // NOTE that most of the Vec<T> are likely to fail
    // will require the followign implementation
    // https://serde.rs/string-or-struct.html
    bin: Option<Vec<String>>,
    depends: Option<Vec<String>>,
    env_add_path: Option<String>,
    // I am not sure if this works the way I hope it works
    // TODO: Test whether this works in accordance to the manifest
    env_set: Option<Vec<(String, String)>>,
    extract_dir: Option<String>,
    extract_to: Option<String>,
    hash: Option<Vec<String>>,
    innosetup: Option<bool>,
    notes: Option<Vec<String>>,
    psmodule: Option<ModuleName>,
    url: Option<Vec<String>>,

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
struct ModuleName {
    name: String,
}
