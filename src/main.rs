mod zipper;

use std::process::Command;
use zipper::unzip;

fn main() {
    println!("Running on {}", std::env::consts::OS);

    let home_dir = get_home_directory().expect("Failed to get home directory").trim().to_owned();
    dbg!(&home_dir);
    let download_dir = home_dir.clone() + "/Downloads/";
    let extract_dir = home_dir.clone() + "/Documents/aleph/";

    let mut filepath = String::new();
    match download_url("https://github.com/lukesampson/cowsay-psh/archive/master.zip", &download_dir) {
        Ok(name) => filepath = name,
        Err(error) => println!("Download Failed with: {error}"),
    };

    match unzip(&filepath, &extract_dir) {
        Ok(directory) => {
            // basically edit the current powershell profile here
        },
        Err(_) => todo!(),
    };
}

fn get_home_directory() -> Option<String> {
    let Ok(output) = Command::new("pwsh")
        .args(["-c", "echo", "$home"])
        .output() 
    else { return None };

    let home_directory = String::from_utf8(output.stdout).unwrap();
    if !home_directory.is_empty()  { Some(home_directory) }
    else { None }
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
