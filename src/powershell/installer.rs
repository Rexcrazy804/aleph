use std::fs;

pub fn append_to_path(home_dir: &str, paths: &Vec<String>) -> std::io::Result<()> {
    let profile_path =
        home_dir.to_owned() + "/Documents/PowerShell/Microsoft.PowerShell_profile.ps1";

    //let profile_path = "./config.ps1";
    let ps_profile = match fs::read_to_string(&profile_path) {
        Ok(content) => content,
        Err(_) => {
            println!("FILE DOES NOT EXIST: {profile_path}");
            todo!("Populate it with the base template so we can write the the file later");
        }
    };

    let mut modified_ps_profile = String::new();
    for line in ps_profile.lines() {
        if line.contains("$env:PATH = (") {
            // TODO if the file does not have a $env:PATH = (
            // i.e. we are touching a profile that was created by the user and not us
            // handling such a situation should be easy I'll leave it to Sanoy :D
            modified_ps_profile.push_str(&(line.to_owned() + "\n"));
            for path in paths {
                let replaced_path = path.replace(home_dir, "$HOME");
                // <space><space>"PATH;" +
                modified_ps_profile
                    .push_str(&("  \"".to_owned() + &replaced_path + ";\"" + " +" + "\n"));
                //TODO remove duplicate paths and preferably notify that the program has already
                //been installed (? dk how that would happen) if there exists a corresponding path
            }
            continue;
        }
        modified_ps_profile.push_str(&(line.to_owned() + "\n"));
    }

    fs::write(profile_path, modified_ps_profile)?;
    println!("installed program to path");
    Ok(())
}
