use std::process::Command;

// actually the only possible way for this to fail is for powershell to not be installed
// in the operating system
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
pub fn download_url(url: &str, download_location: &str) -> Result<String, String> {
    let filename = get_filename(url).unwrap_or("file.zip".to_string());

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

// First we need to get this to be able to extract simple .msi file from Destination to target
// Adjacently we'll need to implemnet a helper funtion called String injector that will be
// repsonbile for replacing powerhsell $variables with corresponding values on the fly
// I am thinking of a function that takes a string and HashMap<"variablename" : "Value">
// with optional fields to then look for and replace $variable instances with their value
pub fn extract_msi(file_path: &str, target_dir: &str) {
    //$MsiPath = 'msiexec.exe'
    //    $ArgList = @('/a', $Path, '/qn', "TARGETDIR=$DestinationPath\SourceDir")
    //}

    let Ok(output) = Command::new("pwsh")
        .args([
            "-c",
            "msiexec.exe",
            "/a",
            file_path,
            "/qn",
            &format!("TARGETDIR={target_dir}"),
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
