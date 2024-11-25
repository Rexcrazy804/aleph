use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Persist {
    Directory(String),             // "directory" [will be copied as is]J
    AliasedDirectory([String; 2]), // [ "directory" "aliasedDir" ] [will be copied as aliasDir]
}
