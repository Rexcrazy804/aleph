// might have to reloate this later
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::PathBuf;

pub fn unzip(archive: &str, extract_location: &str) -> Result<String, String> {
    let Ok(file) = File::open(archive) else {
        return Err("invalid File path".to_string());
    };

    // in case a root folder does not exist in the archive;
    let alt_root_dir = archive
        // this arhive var hold the absolute path
        .split(['/', '\\'])
        .last()
        .unwrap()
        // the archive name will have a .<file extension>, this removes that
        .split('.')
        .next()
        .unwrap()
        .to_owned()
        + "/";
    println!("Extracting {archive}");
    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return Err("Failed to open archive".to_string());
    };

    // TODO cleanup the unwraps here later
    let mut extracted_directory = String::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => {
                if file.is_dir() {
                    extracted_directory = extract_location.to_string() + path.to_str().unwrap();
                } else {
                    extracted_directory =
                        extract_location.to_string() + &alt_root_dir + path.to_str().unwrap();
                }
                PathBuf::from(&extracted_directory)
            }
            None => continue,
        };

        if file.is_dir() {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    let extracted_root_dir = extracted_directory
        .split(['/', '\\'])
        .rev()
        .take_while(|x| !x.contains("aleph"))
        .last()
        .unwrap();
    Ok(extracted_root_dir.to_owned())
}

pub fn unzip_alt(
    file_path: &str,
    extract_directory: &str,
    dir_to_extract: Option<&String>,
) -> String {
    //! WARNING THIS FUNCTION CAN PANIC!
    let archive: Vec<u8> = std::fs::read(file_path).expect("Failed to read file");

    dbg!(file_path);
    let target_dir = match dir_to_extract {
        Some(dir) => extract_directory.to_owned() + dir,
        None => {
            let (folder_name, _file_type) = file_path
                .split('/')
                .last()
                // rare for this to happen .w.
                // TODO brute force this
                .unwrap_or("bob.zip")
                .split_once('.')
                .unwrap();

            extract_directory.to_owned() + folder_name
        }
    };

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single directory, its contents would be extracted instead.
    zip_extract::extract(Cursor::new(archive), &PathBuf::from(&target_dir), true)
        .expect("Failed to extract");

    // TODO do something with dir_to_extract :)
    target_dir
}
