use super::OneOrMany;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Uninstaller {
    file: Option<String>, // Executable to run for uninstallation
    args: Option<OneOrMany<String>>, // Arguments to pass to the uninstaller
    script: Option<Script>, // Custom uninstallation script
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Script(OneOrMany<String>);
