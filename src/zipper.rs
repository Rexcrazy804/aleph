// might have to reloate this later
use sevenz_rust;
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

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

/// unzips file_path to extract_directory/[dirname] and returns the path to extracted directory
/// dir_to_extract variable does nothing right now
pub fn unzip_alt(
    file_path: &str,
    extract_directory: &str,
    dir_to_extract: Option<&String>,
) -> String {
    dbg!(file_path);
    dbg!(extract_directory);

    // I am certain we can skip the make dirname step by reading and tracking the name of the .json
    // file and then modify the function to extract directly to extract_directory instead of making
    // a folder on top of etract dir
    // TODO: add optional argument to explicitly provide filename
    let (file_dir, file_type) = dbg!(make_dirname_and_get_file_type(file_path));
    let target_dir = format!("{extract_directory}{file_dir}");

    // if we are handling a 7z let sevenz handle that
    if file_type == "7z" {
        use_sevenz(file_path, &target_dir);
        return target_dir;
    };

    // if its a zip file, use zip_extract
    let archive: Vec<u8> = std::fs::read(file_path).expect("Failed to read file");
    zip_extract::extract(Cursor::new(archive), &PathBuf::from(&target_dir), true)
        .expect("Failed to extract");

    target_dir
}

/// this fucntion take a file path and returns a directory name based off the file name
/// and the file type of
fn make_dirname_and_get_file_type(file_path: &str) -> (String, String) {
    let file_name = file_path
        .split(['\\', '/']) // fuck windows
        .last()
        // rare for this to happen .w.
        // TODO brute force this
        .unwrap_or("bob.zip");

    let buffer = file_name.split('.');
    let buffer_count = buffer.clone().count();

    let file_dir = buffer
        .clone()
        .take(buffer_count - 1)
        .collect::<String>()
        .replace(".", "_");

    let file_type = buffer.last().unwrap().to_string();

    (file_dir, file_type)
}

fn use_sevenz(file_path: &str, target_dir: &str) {
    sevenz_rust::decompress_file(file_path, target_dir).expect("complete");

    if let Err(e) = strip_directory(target_dir) {
        eprintln!("failed to strip directory with Error: {e}\n");
    };
}

/// if the top level of the given directory contains only a single folder
/// move the contents of that folder onto the parent directory and delete the folder
fn strip_directory(target_dir: &str) -> io::Result<()> {
    use fs::read_dir;

    let target_dir = Path::new(target_dir);
    let entries_count = read_dir(target_dir)?.count();
    if entries_count != 1 {
        return Ok(());
    }

    let lonely_entry = read_dir(target_dir)?.last().unwrap()?;

    if lonely_entry.path().is_dir() {
        for subdir_entry in fs::read_dir(lonely_entry.path())? {
            let subdir_entry = subdir_entry?;
            fs::rename(
                subdir_entry.path(),
                target_dir.join(subdir_entry.file_name()),
            )?
        }
        fs::remove_dir(lonely_entry.path())?
    }

    Ok(())
}
