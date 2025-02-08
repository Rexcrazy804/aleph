use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use crate::powershell::profile_util::append_to_path;

const WGET_ERR: &str = "The term 'wget' is not recognized";
// actually the only possible way for this to fail is for powershell to not be installed
// in the operating system
// COULD USE CMD FOR THIS WITH: echo %APPDATA% and using appdata/roaming/aleph as root directory
/// Returns the home directory of the user executing the program
/// # Panics
/// - powershell not installed
pub fn get_home_directory() -> PathBuf {
    let output = Command::new("pwsh")
        .args(["-c", "echo", "$home"])
        .output()
        .expect("Failed to execute process [is powershell installed?]");

    let home_directory = String::from_utf8(output.stdout).unwrap().trim().to_string();
    PathBuf::from(home_directory)
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
pub fn download_url(
    url: &str,
    download_location: &Path,
    packages_dir: &Path,
) -> Result<PathBuf, String> {
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
        // EXECUTE THIS PORTION IF WGET IS NOT FOUND;
        if stderr.contains(WGET_ERR) {
            let extract_dir = get_wget(packages_dir);
            append_to_path(&get_home_directory(), &vec![extract_dir])
                .expect("failed to append wget to PATH");
            return download_url(url, download_location, packages_dir);
        }

        // Parses the exact file path
        let Some((_url, path)) = stderr.split_once('"') else {
            println!("{stderr}");
            return Err("Failed to parse wget output".to_string());
        };
        let Some((path, _)) = path.trim().split_once('"') else {
            return Err("Failed to parse wget output".to_string());
        };
        // just to not cause weird inconsistencies
        let path = path.replace('/', "\\");

        println!("Succesfully Downloaded to: {path}");
        let archive = PathBuf::from_str(&path).expect("Failed to convert string to path");
        if let Some((_, new_archive_name)) = url.split_once("#/") {
            let new_archive_name = new_archive_name.trim();
            let mut new_archive = archive.clone();
            new_archive.set_file_name(new_archive_name);

            std::fs::rename(&archive, &new_archive).expect("Failed to rename archive");
            return Ok(new_archive);
        }
        return Ok(archive);
    }

    Err("Wget generated no output / Powershell is not installed".to_owned())
}

/// gets the wget executable if wget is missing
/// # Panics
/// - conversion of path to string
#[must_use]
pub fn get_wget(packages_path: &Path) -> PathBuf {
    const VERSION: &str = "1.21.4";
    let arch = match std::env::consts::ARCH {
        "x86" => "32",
        "x86_64" => "64",
        "aarch64" => "a64",
        _ => panic!("Unsupported architecture"),
    };

    let url = format!("https://eternallybored.org/misc/wget/{VERSION}/{arch}/wget.exe");
    let filename = "wget.exe";
    println!("Downloading file {filename}...");

    let extract_dir = packages_path.join("wget").join(VERSION);
    fs::create_dir_all(&extract_dir).expect("Failed to create extract dir");

    // empty to select current directory
    let file_path = extract_dir.join(filename);

    let Ok(_output) = Command::new("pwsh")
        .args([
            "-c",
            "Invoke-WebRequest",
            &url,
            "-OutFile ",
            file_path.to_str().unwrap(),
        ])
        .output()
    else {
        panic!("Failed to execute request");
    };

    extract_dir
}
