use crate::AlephConfig;
use std::fs;

pub fn uninstall_package(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let package_path = config.paths.packages.join(package_name);
    if !package_path.exists() {
        return Err(format!(
            "Package '{}' not found at {:?}",
            package_name, package_path
        ));
    }

    println!("Uninstalling package: {}...", package_name);

    remove_from_profile(config, package_name)?;

    fs::remove_dir_all(&package_path)
        .map_err(|e| format!("Failed to remove package directory: {}", e))?;

    println!("Successfully uninstalled package: {}", package_name);
    Ok(())
}

fn remove_from_profile(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let profile_path = config
        .paths
        .home
        .join("Documents")
        .join("PowerShell")
        .join("Microsoft.PowerShell_profile.ps1");

    let profile_content = fs::read_to_string(&profile_path)
        .map_err(|e| format!("Failed to read PowerShell profile: {}", e))?;

    println!("Original profile content:\n{}", profile_content);

    let search_str = format!("\\{}\\", package_name.trim());
    println!("Filtering out lines containing: {}", search_str);

    let filtered_lines: Vec<String> = profile_content
        .lines()
        .filter(|line| !line.contains(&search_str))
        .map(|line| line.to_string())
        .collect();

    let new_profile = filtered_lines.join("\n");

    println!("New profile content:\n{}", new_profile);

    fs::write(&profile_path, new_profile)
        .map_err(|e| format!("Failed to update PowerShell profile: {}", e))?;

    println!(
        "Removed all lines containing \"{}\" from PowerShell profile.",
        search_str
    );
    Ok(())
}
