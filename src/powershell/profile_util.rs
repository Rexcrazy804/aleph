use crate::AlephConfig;
use std::fs::{self, create_dir};
use std::path::{Path, PathBuf};

// uninstaller
pub fn remove_from_path(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let profile_path = config
        .paths
        .home
        .join("Documents")
        .join("PowerShell")
        .join("Microsoft.PowerShell_profile.ps1");

    let profile_content = fs::read_to_string(&profile_path)
        .map_err(|e| format!("Failed to read PowerShell profile: {e}"))?;

    let search_str = format!("\\{}\\", package_name.trim());

    let filtered_lines: Vec<String> = profile_content
        .lines()
        .filter(|line| !line.contains(&search_str))
        .map(std::string::ToString::to_string)
        .collect();

    let new_profile = filtered_lines.join("\n");

    fs::write(&profile_path, new_profile)
        .map_err(|e| format!("Failed to update PowerShell profile: {e}"))?;

    Ok(())
}

const DEFAULT_PROFILE: &str = "\
# This file is automatically generated by the package manager.
$env:PATH = (
$env:PATH)
";

/// Apends the given ``paths`` to the $env:PATH in the Miscrosoft.PowerShell.prfoile.ps1 file
/// - if the file does not exist a basic template is leveraged
///
/// # Panics
/// - failures to convert the paths to string
///
/// # Errors
/// - IO erros like being unable to write to profile file
pub fn append_to_path(home_dir: &Path, paths: &Vec<PathBuf>) -> std::io::Result<()> {
    let powershell_dir = home_dir.join("Documents\\PowerShell");
    if let Ok(false) = powershell_dir.try_exists() {
        create_dir(powershell_dir.clone()).expect("Failed to create powershell folder");
    }

    let profile_path = powershell_dir.join("Microsoft.PowerShell_profile.ps1");

    let mut ps_profile = if let Ok(content) = fs::read_to_string(&profile_path) {
        content
    } else {
        println!("FILE DOES NOT EXIST: {profile_path:?}");
        String::from(DEFAULT_PROFILE)
    };

    if !ps_profile.contains("$env:PATH = (") {
        ps_profile.push_str(DEFAULT_PROFILE);
    }

    let mut modified_ps_profile = String::new();
    let mut intermediate_path_buffer: Vec<String> = Vec::new();
    let home_dir_str = home_dir
        .to_str()
        .expect("Failed to convert home_dir into str");

    for path in paths {
        let path = path.to_str().expect("Failed to convert path into str");
        let replaced_path = path.replace(home_dir_str, "$HOME");
        intermediate_path_buffer.push("  \"".to_owned() + &replaced_path + ";\"" + " +");
    }

    let mut paths_entry_flag = false;

    for line in ps_profile.lines() {
        if line.contains("$env:PATH = (") {
            paths_entry_flag = true;
            modified_ps_profile.push_str(&(line.to_owned() + "\n"));
            continue;
        }
        if line.contains("$env:PATH)") {
            paths_entry_flag = false;
            intermediate_path_buffer.sort();
            intermediate_path_buffer.dedup_by(|a, b| {
                if a == b {
                    println!("path {a} already installed ");
                    true
                } else {
                    false
                }
            });
            for line in intermediate_path_buffer.clone() {
                modified_ps_profile.push_str(&(line + "\n"));
            }
        }

        if paths_entry_flag {
            intermediate_path_buffer.push(line.to_owned());
        } else {
            modified_ps_profile.push_str(&(line.to_owned() + "\n"));
        }
    }

    fs::write(profile_path, modified_ps_profile)?;
    Ok(())
}
