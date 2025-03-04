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

pub struct NormalizedShortCuts<'a> {
    pub target: &'a String,
    pub label: &'a String,
    pub args: Option<&'a String>,
    pub icon: Option<&'a String>,
}

impl Shortcuts {
    #[must_use]
    pub fn normalize(&self) -> NormalizedShortCuts {
        match self {
            Shortcuts::Standard([target, label]) => NormalizedShortCuts {
                target,
                label,
                args: None,
                icon: None,
            },
            Shortcuts::WithArgs([target, label, args]) => NormalizedShortCuts {
                target,
                label,
                args: Some(args),
                icon: None,
            },
            Shortcuts::WithIcon([target, label, args, icon]) => NormalizedShortCuts {
                target,
                label,
                args: Some(args),
                icon: Some(icon),
            },
        }
    }
}
