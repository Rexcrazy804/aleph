use crate::{powershell::uninstaller::remove_from_profile, AlephConfig};
use std::fs;

pub fn remove_package_dir(
    config: &AlephConfig,
    package_name: Option<&String>,
) -> Result<(), String> {
    // Unwrap the Option to get a &String, and then trim it to get a &str.
    let package_name = package_name.ok_or("Package name required for uninstall.".to_string())?;
    let package_name = package_name.trim(); // Now package_name is a &str

    let package_path = config.paths.packages.join(package_name);
    if let Ok(false) = package_path.try_exists() {
        return Err(format!(
            "Package '{}' not found at {:?}",
            package_name, package_path
        ));
    }

    remove_from_profile(config, package_name)?;

    fs::remove_dir_all(&package_path)
        .map_err(|e| format!("Failed to remove package directory: {}", e))?;

    println!("Successfully uninstalled package: {}", package_name);
    Ok(())
}
