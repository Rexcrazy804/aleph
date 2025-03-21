use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Binary {
    // "program.exe"
    Executable(String),
    // ["p1.exe", "p2.exe"] || handled differently when nested inside AliasedExecutables
    Executables(Vec<String>),
    /*
    [
       "p1.exe",
       "p2.exe",
       # I don't think we can handle this for now
       [ "program.exe" "alias" "--argument1" ... ]
    ]
    */
    AliasedExecutables(Vec<Binary>),
}

impl Binary {
    fn extract_dir_or_none(path: &str) -> Option<String> {
        let path_count = path.split('\\').count();
        if path_count == 1 {
            return None;
        }

        Some(
            path.split('\\')
                .enumerate()
                .take_while(|(index, _)| *index < path_count - 1)
                .fold(String::new(), |acc, (_, data)| acc + data + "\\"),
        )
    }

    #[must_use]
    pub fn normalized_executable_directores(&self, package_path: &PathBuf) -> Vec<PathBuf> {
        match &self {
            Binary::Executable(path) => {
                if let Some(dir) = Self::extract_dir_or_none(path) {
                    return vec![package_path.join(dir)];
                }
            }
            Binary::Executables(paths) => {
                let mut output = Vec::new();
                for path in paths {
                    if let Some(dir) = Self::extract_dir_or_none(path) {
                        output.push(package_path.join(dir));
                    }
                }
                return output;
            }
            Binary::AliasedExecutables(binary_vec) => {
                let mut output = Vec::new();
                for binary in binary_vec {
                    output.append(&mut binary.normalized_executable_directores(package_path));
                }
                return output;
            }
        }

        vec![]
    }
}
