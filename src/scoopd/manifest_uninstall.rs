use crate::AlephConfig;
use std::fs;
use std::path::PathBuf;

pub fn uninstall_package(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let package_path = config.paths.packages.join(package_name);
    if !package_path.exists() {
        return Err(format!(
            "Package '{}' not found at {:?}",
            package_name, package_path
        ));
    }

    println!("Uninstalling package: {}...", package_name);

    remove_from_profile(config, &package_path)?;

    fs::remove_dir_all(&package_path)
        .map_err(|e| format!("Failed to remove package directory: {}", e))?;

    println!("Successfully uninstalled package: {}", package_name);
    Ok(())
}

fn remove_from_profile(config: &AlephConfig, package_path: &PathBuf) -> Result<(), String> {
    let profile_path = config
        .paths
        .home
        .join("Documents")
        .join("Powershell")
        .join("Microsoft.PowerShell_profile.ps1");

    let profile_content = fs::read_to_string(&profile_path)
        .map_err(|e| format!("Failed to read Powershell profile: {}", e))?;

    let package_path_str = package_path
        .to_str()
        .ok_or("Failed to convert package path to string")?;

    let new_profile = profile_content
        .lines()
        .filter(|line| !line.contains(package_path_str))
        .collect::<Vec<&str>>()
        .join("\n");

    fs::write(&profile_path, new_profile)
        .map_err(|e| format!("Failed to update Powershell profile: {}", e))?;

    println!(
        "Removed package path from PowerShell profiel: {:?}",
        package_path
    );
    Ok(())
}
