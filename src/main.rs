mod zipper;

use std::{fs, process::Command};
use zipper::unzip;

fn main() {
    println!("Running on {}", std::env::consts::OS);

    let home_dir = get_home_directory()
        .expect("Failed to get home directory, perhaps powershell is not installed?")
        // we can't do anything if powershell itself is not present. maybe if I can get something
        // like reqwuest to work correctly maybe then we'll have a shot?
        .trim()
        .to_owned();

    let download_dir = home_dir.clone() + "/Downloads/";
    let extract_dir = home_dir.clone() + "/Documents/aleph/";

    let mut filepath = String::new();
    match download_url(
        // I guess the next step is to get this link from the scoop manifest
        "https://github.com/jftuga/less-Windows/releases/download/less-v661/less-x64.zip",
        &download_dir,
    ) {
        Ok(name) => filepath = name,
        Err(error) => println!("Download Failed with: {error}"),
    };

    let unziped_dir = unzip(&filepath, &extract_dir).expect("Failed to extract");

    powershell_path_install(&home_dir, &vec![extract_dir + &unziped_dir + "/"])
        .unwrap_or_else(|e| panic!("{e}"));
}

fn get_home_directory() -> Option<String> {
    let Ok(output) = Command::new("pwsh").args(["-c", "echo", "$home"]).output() else {
        return None;
    };

    let home_directory = String::from_utf8(output.stdout).unwrap();
    if !home_directory.is_empty() {
        Some(home_directory)
    } else {
        None
    }
}

/// downloads the given url and returns the path of the downloaded file
fn download_url(url: &str, download_location: &str) -> Result<String, String> {
    let filename = get_filename(url).unwrap_or("file.bin".to_string());

    println!("Downloading file {filename}");

    // empty to select current directory
    let file_path = download_location.to_string() + &filename;

    let Ok(output) = Command::new("pwsh")
        .args(["-c", "Invoke-WebRequest", url, "-OutFile ", &file_path])
        .output()
    else {
        return Err("Failed to execute request".to_string());
    };

    match String::from_utf8(output.stderr) {
        Ok(stderr) => {
            if stderr.is_empty() {
                println!("Download Sucessfull");
                Ok(file_path)
            } else {
                Err(stderr)
            }
        }
        Err(_) => Err("Failed to parse stderr".to_string()),
    }
}

fn get_filename(url: &str) -> Option<String> {
    let last_token = url.split('/').last()?;

    if last_token.contains('.') {
        Some(last_token.to_string())
    } else {
        None
    }
}
fn powershell_path_install(home_dir: &str, paths: &Vec<String>) -> std::io::Result<()> {
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
            // handle such a situation should be easy I'll leave it to Sanoy :D
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
