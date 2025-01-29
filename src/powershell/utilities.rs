use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

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
/// downloaded file.
/// # Panics
/// - Unable to convert the ``download_location`` to a string
/// - Unable to convert the string containing the path to the downloaded file into a ``PathBuf``
///
/// # Errors
/// - Failure to run powershell (*powershell is not installed*)
/// - Failure to find *wget.exe* in PATH
/// - Invalid url
pub fn download_url(url: &str, download_location: &Path) -> Result<PathBuf, String> {
    println!("Downloading file {url}...");

    let Ok(output) = Command::new("pwsh")
        .args([
            "-c",
            "wget",
            "-nv",
            url,
            "-P",
            download_location
                .to_str()
                .expect("Failed to convert location to String"),
        ])
        .output()
    else {
        // TODO in the event that this fails try using powershell's invoke web request [previous
        // method] to download wget and append it to path and retry the download. Sanoy you can
        // give this a shot
        return Err("Failed to execute request".to_owned());
    };

    // dk why but wget writes to stderr by default .w.
    if let Ok(stderr) = String::from_utf8(output.stderr) {
        let Some((_url, path)) = stderr.split_once("->") else {
            return Err("Failed to parse wget output".to_string());
        };
        let Some((path, _)) = path.trim().split_once(' ') else {
            return Err("Failed to parse wget output".to_string());
        };

        // just to not cause weird inconsistencies
        let path = path.trim_matches('"').replace('/', "\\");
        println!("Succesfully Downloaded to: {path}");
        let archive = PathBuf::from_str(&path).expect("Failed to convert string to path");
        return Ok(archive);
    }

    Err("Wget generated no output / Powershell is not installed".to_owned())
}
