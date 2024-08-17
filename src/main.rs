use std::fs::{self, File};
use std::io;
use std::process::Command;

fn main() {
    println!("Hello, Za WARUDO!");
    println!("I am totally running on {}", std::env::consts::OS);

    let mut filepath = String::new();
    match download_url("https://github.com/lukesampson/cowsay-psh/archive/master.zip") {
        Ok(name) => filepath = name,
        Err(error) => println!("Download Failed with: {error}"),
    };


    // TODO try to unzip this shit
    let _ = unzip(&filepath);
}

fn unzip(archive: &str) -> Result<(), String> {
    let Ok(file) = File::open(archive) else {
        return Err("invalid File path".to_string());
    };

    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return Err("Failed to open archive".to_string());
    };

    // TODO cleanup the unrwaps here later
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    Ok(())
}

/// downloads the given url and returns the path of the downloaded file
fn download_url(url: &str) -> Result<String, String> {
    let filename = get_filename(url).unwrap_or("file.bin".to_string());

    println!("Downloading file {filename}");

    // empty to select current directory
    let download_location: String = String::from("downloads/");
    let file_path = download_location + &filename;

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
