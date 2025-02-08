use crate::{powershell::profile_util::remove_from_path, AlephConfig};
use std::fs;

pub fn manifest_uninstaller(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let package_path = config.paths.packages.join(package_name);
    if let Ok(false) = package_path.try_exists() {
        return Err(format!("Package '{package_name}' not found"));
    }

    remove_from_path(config, package_name)?;

    fs::remove_dir_all(&package_path)
        .map_err(|e| format!("Failed to remove package directory: {e}"))?;

    println!("\x1b[92muninstalled {package_name}\x1b[0m");
    Ok(())
}
