use std::fs;

//Remove specified paths from the user's Powershell environment profile.
pub fn remove_from_path(home_dir: &str, path: &Vec<String>) -> std::io::Result<()> {
    let profile_path=
        home_dir.to_owned() + "/Documents/Powershell/Microsoft.PowerShell_profile.ps1";

    //Check if the profile file exists
    if !fs::metadata(&profile_path).is_ok() {
        println!("PowerShell profile does not exist: {profile_path}");
        return Ok(());
    }

    // Read the existing PowerShell profile
    let mut ps_profile = fs::read_to_string(&profile_path)?;

    if !ps_profile.contains("$env:PATH = (") {
        println!("No PATH modifications found in the profile.");
        return Ok(()); // Exit if no PATH block is present
    }

    let mut modified_ps_profile = String::new();
    let mut paths_to_remove: Vec<String> = path
        .iter()
        .map(|path| path.replace(home_dir, "$HOME"))
        .collect();
    paths_to_remove.sort();

    let mut paths_entry_flag = false;
    let mut intermediate_path_buffer: Vec<String> = Vec::new();

    for line in ps_profile.lines() {
        if line.contains("$env:PATH = (") {
            paths_entry_flag = true;
            modified_ps_profile.push_str(&(line.to_owned() + "\n"));
            continue;
        }

        if line.contains("$env:PATH)") {
            paths_entry_flag = false;

            // Remove matching paths
            intermediate_path_buffer.retain(|existing_path| {
                let should_keep = !paths_to_remove.iter().any(|path_to_remove| {
                    existing_path.trim_end_matches(" +").trim() == path_to_remove
                });
                if !should_keep {
                    println!("Removed PATH entry: {}", existing_path);
                }
                should_keep
            });

            for line in &intermediate_path_buffer {
                modified_ps_profile.push_str(&(line.to_owned() + "\n"));
            }
        }

        if paths_entry_flag {
            intermediate_path_buffer.push(line.to_owned());
        } else {
            modified_ps_profile.push_str(&(line.to_owned() + "\n"));
        }
    }

    // Write the updated profile back to the file
    fs::write(profile_path, modified_ps_profile)?;
    println!("Updated PowerShell profile to remove paths.");
    Ok(())
}


