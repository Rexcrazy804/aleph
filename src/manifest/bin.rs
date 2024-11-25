use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Binary {
    // "program.exe"
    Executable(String),
    // ["p1.exe", "p2.exe"] || handled differently when nested inside AliasedExecutables
    Executables(Vec<String>),
    // [ "program.exe" "alias" "--argument1" ... ]
    AliasedExecutables(Vec<Binary>),
}
