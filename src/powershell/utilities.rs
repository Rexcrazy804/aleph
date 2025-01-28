use std::path::{Path, PathBuf};
use std::process::Command;

// actually the only possible way for this to fail is for powershell to not be installed
// in the operating system
// COULD USE CMD FOR THIS WITH: echo %APPDATA% and using appdata/roaming/aleph as root directory
pub fn get_home_directory() -> String {
    let output = Command::new("pwsh")
        .args(["-c", "echo", "$home"])
        .output()
        .expect("Failed to execute process [is powershell installed?]");

    let home_directory = String::from_utf8(output.stdout).unwrap().trim().to_string();
    home_directory
}

/// attempts to download the given url to the provided directory and returns the path to the
/// downloaded file. TODO correct the return type to be a a std::path::PATH
pub fn download_url(url: &str, download_location: &Path) -> Result<PathBuf, String> {
    println!("Downloading file {url}");

    let Ok(output) = Command::new("pwsh")
        .args(["-c", "wget", url, "-P", download_location.to_str().unwrap()])
        .output()
    else {
        return Err("Failed to execute request".to_owned());
    };

    if let Ok(stdout) = String::from_utf8(output.stdout) {
        println!("{stdout}");
        println!("NO STDOUT");
    } else {
        println!("NO STDOUT");
    }

    //match String::from_utf8(output.stderr) {
    //    Ok(stderr) => {
    //        if stderr.is_empty() {
    //            println!("Download Sucessfull");
    //            Ok(file_path)
    //        } else {
    //            Err(stderr)
    //        }
    //    }
    //    Err(_) => Err("Failed to parse stderr".to_string()),
    //}

    Err("ehe".to_owned())
}

fn get_filename(url: &str) -> Option<String> {
    let last_token = url.split('/').last()?;

    if last_token.contains('.') {
        Some(last_token.to_string())
    } else {
        None
    }
}

// First we need to get this to be able to extract simple .msi file from Destination to target
// Adjacently we'll need to implemnet a helper funtion called String injector that will be
// repsonbile for replacing powerhsell $variables with corresponding values on the fly
// I am thinking of a function that takes a string and HashMap<"variablename" : "Value">
// with optional fields to then look for and replace $variable instances with their value
pub fn extract_msi(file_path: &str, target_dir: &str) {
    println!("WARN support for msi installation is incomplete!");
    let Ok(output) = Command::new("pwsh")
        .args([
            "-c",
            "msiexec.exe",
            "/i",
            file_path,
            "/qn",
            &format!("INSTALLDIR={target_dir}"),
        ])
        .output()
    else {
        panic!("Failed to execute request");
    };

    println!("{}", String::from_utf8(output.stdout).unwrap());
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
    }
}
