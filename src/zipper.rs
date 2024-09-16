use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

pub fn unzip(archive: &str, extract_location: &str) -> Result<String, String> {
    let Ok(file) = File::open(archive) else {
        return Err("invalid File path".to_string());
    };

    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return Err("Failed to open archive".to_string());
    };

    // TODO cleanup the unrwaps here later
    let mut exe_dir = String::new();
    for i in 0..archive.len() { 
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => {
                exe_dir = extract_location.to_string() + path.to_str().unwrap();
                PathBuf::from(&exe_dir)
            },
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
    Ok(exe_dir)
}

