use serde::{Deserialize, Serialize};

// NOTE: I really don't think I want to implement this
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Shortcuts {
    // First field: Path to target
    // Second field: Label
    // (optional) Third field: star parametres (?) args maybe?
    // (optional) Fourth field: path to icon
    Standard([String; 2]),
    WithArgs([String; 3]),
    WithIcon([String; 4]),
}
