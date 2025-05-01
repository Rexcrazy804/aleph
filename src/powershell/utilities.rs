use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use crate::cli::subcommands::find_package;
use crate::manifest::shortcuts::{NormalizedShortCuts, Shortcuts};
use crate::manifest::Manifest;
use crate::powershell::profile_util::append_to_path;
use crate::AlephConfig;

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
            //"-N", this is great but yeah if the file exists => no output from wget :(
            // so till I figure something out we're stuck with this D:
            "-nv",
            url,
            "-P",
            &format!("'{}'", download_location.display()),
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
            println!("parse failure 1 {stderr}");
            println!("{:?}", String::from_utf8(output.stdout).unwrap());
            return Err("Failed to parse wget output".to_string());
        };
        let Some((path, _)) = path.trim().split_once('"') else {
            println!("parse failure 2 {stderr}");
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
            // fixes username having a space
            &format!("'{}'", file_path.display()),
        ])
        .output()
    else {
        panic!("Failed to execute request");
    };

    extract_dir
}

pub fn create_shortcuts(
    shortcuts: &[Shortcuts],
    package_dir: &Path,
    home_dir: &Path,
) -> Result<(), String> {
    let shortcuts_path = PathBuf::from(home_dir)
        .join("AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\AlephPrograms\\");

    if let Ok(false) = shortcuts_path.try_exists() {
        std::fs::create_dir_all(&shortcuts_path).expect("Failed to create Directory");
    }

    let create_shorcut = |shortcut: &NormalizedShortCuts| {
        let target = shortcut.target;
        let label = shortcut.label;
        let args = if let Some(args) = shortcut.args {
            &format!("$Shortcut.Arguments = '{args}'")
        } else {
            ""
        };
        let icon = if let Some(icon) = shortcut.icon {
            &format!(
                "$Shortcut.IconLocation = '{}'",
                package_dir.join(icon).to_str().unwrap()
            )
        } else {
            ""
        };
        // define scuffed
        // Don't ask me how long it took to figure this crap out
        // and don't ask me what .'i'nk  files are. KMS
        let powershellargs = [
            "-c ",
            "& {",
            &format!(
                "
                $WshShell = New-Object -COMObject WScript.Shell
                $Shortcut = $WshShell.CreateShortcut('{}.lnk')
                $Shortcut.TargetPath = '{}'
                $Shortcut.WorkingDirectory = '{}'
                {args}
                {icon}
                $Shortcut.Save()
            ",
                shortcuts_path.join(label).to_str().unwrap(),
                package_dir.join(target).to_str().unwrap(),
                package_dir.to_str().unwrap(),
            ),
            "}",
        ];
        Command::new("pwsh").args(powershellargs).output()
    };

    let mut errors = String::new();
    for shortcut in shortcuts {
        let shortcut = shortcut.normalize();
        let output = create_shorcut(&shortcut).expect("Failed to create shorcut: ");
        //println!("{}", String::from_utf8(output.stdout).unwrap());
        //println!("{}", String::from_utf8(output.stderr).unwrap());
        if let Some(code) = output.status.code() {
            if code == 1 {
                eprintln!("Error creating shortcut {}", shortcut.label);
                errors = errors + shortcut.label + " ";
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Removes shortcuts for the given package from the Start Menu
pub fn remove_shortcuts(
    config: &AlephConfig,
    package_name: &str,
    home_dir: &Path,
) -> Result<(), String> {
    let shortcuts_path = home_dir
        .join("AppData")
        .join("Roaming")
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("AlephPrograms");

    if !shortcuts_path.exists() {
        println!("No shortcuts folder found at {:?}", shortcuts_path);
        return Ok(());
    }

    // Get the list of shortcuts for this package
    let shortcuts = get_package_shortcuts(config, package_name)?;

    let mut errors = String::new();
    let mut removed_count = 0;

    for shortcut in shortcuts {
        let shortcut_path = match shortcut {
            Shortcuts::Standard([target, _])
            | Shortcuts::WithArgs([target, _, _])
            | Shortcuts::WithIcon([target, _, _, _]) => {
                shortcuts_path.join(format!("{}.lnk", target))
            }
        };

        if shortcut_path.exists() {
            match fs::remove_file(&shortcut_path) {
                Ok(_) => {
                    println!("Removed shortcut: {:?}", shortcut_path);
                    removed_count += 1;
                }
                Err(e) => {
                    let error_msg = format!("Failed to remove shortcut {:?}: {}", shortcut_path, e);
                    eprintln!("{}", error_msg);
                    errors.push_str(&error_msg);
                    errors.push(' ');
                }
            }
        } else {
            println!("Shortcut not found: {:?}", shortcut_path);
        }
    }

    println!(
        "Removed {} shortcuts for package: {}",
        removed_count, package_name
    );

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn get_package_shortcuts(
    config: &AlephConfig,
    package_name: &str,
) -> Result<Vec<Shortcuts>, String> {
    // Fetch the package manifest file path
    let manifest_path = find_package(config, package_name)
        .ok_or_else(|| format!("Package '{}' not found.", package_name))?;

    // Read and parse the manifest JSON file
    let manifest_content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest file: {}", e))?;
    let manifest: Manifest = serde_json::from_str(&manifest_content)
        .map_err(|e| format!("Failed to parse manifest JSON: {}", e))?;

    // Ensure the manifest contains shortcuts
    let shortcuts = manifest.shortcuts.unwrap_or_else(Vec::new); // Unwraps `Option<Vec<Shortcuts>>`

    if shortcuts.is_empty() {
        return Err(format!("No shortcuts found for package '{}'", package_name));
    }

    Ok(shortcuts)
}
