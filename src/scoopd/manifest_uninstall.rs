use crate::{
    manifest::{bin::Binary, Manifest, OneOrMany},
    powershell::{uninstaller::remove_from_path, utilities::get_home_directory},
};
use std::fs;

/// Uninstalls a package by removing its files/directories and cleaning up environment paths.
pub fn manifest_uninstall(manifest: &Manifest) -> Result<(), String> {
    let home_dir = get_home_directory();
    let extract_dir = format!("{home_dir}\\Documents\\aleph\\");

    // Collect paths to remove
    let mut paths_to_remove: Vec<String> = Vec::new();

    // Handle extract_dir logic
    match &manifest.extract_dir {
        Some(OneOrMany::One(dir)) => {
            let target_dir = format!("{extract_dir}{}", dir);
            paths_to_remove.push(target_dir.clone());
            remove_directory(&target_dir)?;
        }
        Some(OneOrMany::Many(dirs)) => {
            for dir in dirs {
                let target_dir = format!("{extract_dir}{}", dir);
                paths_to_remove.push(target_dir.clone());
                remove_directory(&target_dir)?;
            }
        }
        None => {
            println!("No directories specified in manifest for removal.");
        }
    }

    // Handle executable cleanup
    if let Some(bin_attr) = &manifest.bin {
        let exec_paths = get_executables(&extract_dir, bin_attr)?;
        paths_to_remove.extend(exec_paths);
    }

    // Remove paths from PowerShell profile
    if !paths_to_remove.is_empty() {
        println!("Removing paths from PowerShell profile...");
        remove_from_path(&home_dir, &paths_to_remove).map_err(|e| e.to_string())?;
    }

    println!("Successfully uninstalled package");
    Ok(())
}

/// Removes a directory and its contents, handling errors gracefully.
fn remove_directory(path: &str) -> Result<(), String> {
    match fs::remove_dir_all(path) {
        Ok(_) => {
            println!("Removed directory: {}", path);
            Ok(())
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("Directory not found: {}", path);
            Ok(())
        }
        Err(e) => Err(format!("Failed to remove directory {}: {}", path, e)),
    }
}

/// Retrieves executables listed in the manifest's `bin` field.
fn get_executables(base_dir: &str, bin_attr: &Binary) -> Result<Vec<String>, String> {
    let mut binary_paths: Vec<String> = Vec::new();

    match bin_attr {
        Binary::Executable(exe) => {
            binary_paths.append(&mut find_binaries(base_dir, &[exe.to_string()]));
        }
        Binary::Executables(exes) => {
            binary_paths.append(&mut find_binaries(base_dir, exes));
        }
        Binary::AliasedExecutables(aliased_exes) => {
            for alias_or_exe in aliased_exes {
                match alias_or_exe {
                    Binary::Executable(exe) => {
                        binary_paths.append(&mut find_binaries(base_dir, &[exe.to_string()]));
                    }
                    Binary::Executables(exes) => {
                        binary_paths.append(&mut find_binaries(base_dir, exes));
                    }
                    Binary::AliasedExecutables(vec) => {
                        for alias in vec {
                            binary_paths.append(&mut find_binaries(
                                base_dir,
                                &[match alias {
                                    Binary::Executable(exe) => exe.clone(),
                                    _ => {
                                        return Err(
                                            "Unsupported alias type for conversion".to_string()
                                        )
                                    }
                                }],
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(binary_paths)
}

/// Finds binaries within the specified search directory.
fn find_binaries(search_dir: &str, binaries: &[String]) -> Vec<String> {
    let mut path_to_binary_parent_dirs: Vec<String> = Vec::new();
    path_to_binary_parent_dirs.push(search_dir.to_string());

    for binary in binaries {
        let parent_dir = binary.split('\\').rev().skip(1).collect::<String>();
        if parent_dir.is_empty() {
            continue;
        }

        let full_path_to_parent = format!("{search_dir}\\{parent_dir}");
        path_to_binary_parent_dirs.push(full_path_to_parent);
    }

    path_to_binary_parent_dirs.sort();
    path_to_binary_parent_dirs.dedup();
    path_to_binary_parent_dirs
}
