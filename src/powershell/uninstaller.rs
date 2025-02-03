use crate::AlephConfig;
use std::fs;

pub fn remove_from_profile(config: &AlephConfig, package_name: &str) -> Result<(), String> {
    let profile_path = config
        .paths
        .home
        .join("Documents")
        .join("PowerShell")
        .join("Microsoft.PowerShell_profile.ps1");

    let profile_content = fs::read_to_string(&profile_path)
        .map_err(|e| format!("Failed to read PowerShell profile: {}", e))?;

    let search_str = format!("\\{}\\", package_name.trim());

    let filtered_lines: Vec<String> = profile_content
        .lines()
        .filter(|line| !line.contains(&search_str))
        .map(|line| line.to_string())
        .collect();

    let new_profile = filtered_lines.join("\n");

    fs::write(&profile_path, new_profile)
        .map_err(|e| format!("Failed to update PowerShell profile: {}", e))?;

    Ok(())
}
