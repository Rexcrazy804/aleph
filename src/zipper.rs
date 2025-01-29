// might have to reloate this later
use crate::manifest::OneOrMany;
use crate::powershell;
use sevenz_rust;
use std::fs::{self};
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

/// unzips file_path to extract_directory/[dirname] and returns the path to extracted directory
/// dir_to_extract variable does nothing right now
pub fn unzip_alt(
    archive: &Path,
    extract_directory: &Path,
    #[allow(unused_variables)] dir_to_extract: Option<OneOrMany<String>>,
) -> std::io::Result<()> {
    unimplemented!("Fix the corresponding functions .w.");
    // I am certain we can skip the make dirname step by reading and tracking the name of the .json
    // file and then modify the function to extract directly to extract_directory instead of making
    // a folder on top of etract dir
    // TODO: add optional argument to explicitly provide filename
    let (file_dir, file_type) = make_dirname_and_get_file_type(archive);

    match file_type.as_str() {
        "7z" => extract_7z(archive, extract_directory),
        "msi" => {
            powershell::utilities::extract_msi(archive, extract_directory);
            let _ = strip_directory(extract_directory);
        }
        "zip" => extract_zip(archive, extract_directory),
        _ => panic!("Unsupported File Format!"),
    };

    Ok(())
}

fn extract_zip(file_path: &str, target_dir: &String) {
    let archive: Vec<u8> = std::fs::read(file_path).expect("Failed to read file");
    zip_extract::extract(Cursor::new(archive), &PathBuf::from(target_dir), true)
        .expect("Failed to extract");
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

fn extract_7z(file_path: &str, target_dir: &str) {
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
