use std::process::Command;
//use std::path::Path;

const DEBUG: bool = true;

fn main() {
    println!("Hello, Za WARUDO!");
    println!("I am totally running on {}", std::env::consts::OS);

    match download_url("https://github.com/lukesampson/cowsay-psh/archive/master.zip") {
        Ok(()) => println!("Download succesfull"),
        Err(error) => println!("Download Failed with: {error}"),
    };
}

fn download_url(url: &str) -> Result<(), String> {
    //! downloads the given url and returns the path of the downloaded file

    let Some(filename) = get_filename(url) else {
        return Err("Failed to extract file name".to_string());
    };

    if DEBUG {
        println!("Downloading file {filename}")
    }

    // empty to select current directory
    let download_location: String = String::from("");
    let file_path = download_location + &filename;

    let Ok(output) = Command::new("pwsh")
        .args(dbg!(["-c", "Invoke-WebRequest", url, "-OutFile ", &file_path]))
        .output()
    else {
        return Err("Failed to execute request".to_string());
    };

    match String::from_utf8(output.stderr) {
        Ok(str) => {
            if str.is_empty() {
                if DEBUG {
                    println!("Download Sucessfull")
                }
                Ok(())
            } else {
                Err(str)
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
