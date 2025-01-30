// might have to reloate this later
use crate::manifest::OneOrMany;
use sevenz_rust;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

/// unzips ``file_path`` to ``extract_directory``
/// *WARN* ``dir_to_extract`` variable does nothing right now
/// # Panics
/// - Unable to detect the file type of the archive
/// - Encounters unsupported archive format
/// - unable to create disgnated ``extract_directory``
/// - unable to delete downloaded archive (*maybe we should move it else where for caching?*)
pub fn extract_archive(
    archive: &Path,
    extract_directory: &Path,
    #[allow(unused_variables)] dir_to_extract: Option<&OneOrMany<String>>,
) {
    // I am certain we can skip the make dirname step by reading and tracking the name of the .json
    // file and then modify the function to extract directly to extract_directory instead of making
    // a folder on top of etract dir
    // TODO: add optional argument to explicitly provide filename
    use fs::{create_dir_all, remove_file};
    let file_type = archive
        .extension()
        .and_then(OsStr::to_str)
        .expect("Failed to detect File Extension");

    create_dir_all(extract_directory).expect("Failed to create directory");
    println!("Created package directory");

    match file_type {
        "7z" => extract_7z(archive, extract_directory),
        "msi" => {
            extract_msi(archive, extract_directory);
        }
        "zip" => extract_zip(archive, extract_directory),
        _ => {
            remove_file(archive).expect("Failed to remove downloaded archive");
            panic!("Unsupported File Format!");
        }
    };
    println!("Extracted archive successfully");

    let _ = strip_directory(extract_directory, dir_to_extract);
    remove_file(archive).expect("Failed to remove downloaded archive");
}

fn extract_zip(archive: &Path, extract_dir: &Path) {
    let archive: Vec<u8> = std::fs::read(archive).expect("Failed to read file");
    zip_extract::extract(Cursor::new(archive), &PathBuf::from(extract_dir), true)
        .expect("Failed to extract");
}

fn extract_7z(archive: &Path, extract_dir: &Path) {
    sevenz_rust::decompress_file(archive, extract_dir).expect("Failed to decompress 7z archive");
}

/// A swift implimentation to extract the contents of the msi file into the given directory
/// # Panics
/// The program will panic if it fails to convert the archive path to a string
pub fn extract_msi(archive: &Path, extract_dir: &Path) {
    // WARN FATAL msiexec won't extract the files into the specified directory if the msi has
    // already been installed (i.e.) registered in the windows uninstaller. we needa figure out how
    // to unregister it from there sto be able to support multiple versions for .msi files

    use std::process::Command;
    let archive = archive
        .to_str()
        .expect("Failed to convert Extract Dir to String");
    let extract_dir = extract_dir
        .to_str()
        .expect("Failed to convert Extract Dir to String")
        .to_owned()
        + "\\";

    println!("WARN support for msi installation is incomplete!");
    let Ok(output) = Command::new("pwsh")
        .args([
            "-c",
            "msiexec.exe",
            "/i",
            archive,
            "/qn",
            &format!("INSTALLDIR={extract_dir}"),
        ])
        .output()
    else {
        panic!("Failed to execute request");
    };

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

/// if the top level of the given directory contains only a single folder
/// move the contents of that folder onto the parent directory and delete the folder
/// this function repeats the above process recursively till the directory has more than only a
/// single entry [folder or file name]
/// ``dir_to_extract`` comes from ``Manifest.extract_dir`` and is not used as of now
#[allow(clippy::only_used_in_recursion)]
fn strip_directory(
    extract_dir: &Path,
    dir_to_extract: Option<&OneOrMany<String>>,
) -> io::Result<()> {
    use fs::read_dir;

    let target_dir = Path::new(extract_dir);
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
            )?;
        }
        fs::remove_dir(lonely_entry.path())?;
    }

    strip_directory(extract_dir, dir_to_extract)
}
