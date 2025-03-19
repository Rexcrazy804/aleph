use crate::powershell::utilities::get_home_directory;
use crate::powershell::utilities::remove_shortcuts;
use crate::{powershell::profile_util::remove_from_path, AlephConfig};
use std::fs;

pub fn manifest_uninstaller(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let package_path = config.paths.packages.join(package_name);
    if let Ok(false) = package_path.try_exists() {
        return Err(format!("Package '{package_name}' not found"));
    }

    remove_from_path(config, &vec![package_name], false)?;
    let home_directory = get_home_directory();
    remove_shortcuts(&config, package_name, &home_directory)?;
    fs::remove_dir_all(&package_path)
        .map_err(|e| format!("Failed to remove package directory: {e}"))?;

    println!("\x1b[92muninstalled {package_name}\x1b[0m");
    Ok(())
}
