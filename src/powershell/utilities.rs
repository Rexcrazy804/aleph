use std::process::Command;

pub fn get_home_directory() -> Option<String> {
    let Ok(output) = Command::new("pwsh").args(["-c", "echo", "$home"]).output() else {
        return None;
    };

    let home_directory = String::from_utf8(output.stdout).unwrap().trim().to_string();
    if !home_directory.is_empty() {
        Some(home_directory)
    } else {
        None
    }
}

pub fn download_url(url: &str, download_location: &str) -> Result<String, String> {
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
