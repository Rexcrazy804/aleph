use crate::errors::extraction::ExtractError;
use crate::manifest::OneOrMany;
use crate::scoopd::manifest_install::dependency_install;
use crate::AlephConfig;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

/// unzips ``file_path`` to ``extract_directory``
/// *WARN* ``dir_to_extract`` variable does nothing right now
/// # Errors
/// - most important error being failure to find 7zip in path
/// - rest are well described in the ``ExtractError`` definition
pub fn extract_archive(
    config: &AlephConfig,
    archive: &Path,
    extract_directory: &Path,
    #[allow(unused_variables)] dir_to_extract: Option<&OneOrMany<String>>,
) -> Result<(), ExtractError> {
    // I am certain we can skip the make dirname step by reading and tracking the name of the .json
    // file and then modify the function to extract directly to extract_directory instead of making
    // a folder on top of etract dir
    // TODO: add optional argument to explicitly provide filename
    use fs::{create_dir_all, remove_file};
    let file_type = archive
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(ExtractError::NoFileExtensionError)?;

    create_dir_all(extract_directory)?;
    println!("Created package directory");

    match file_type {
        "7z" | "zip" | "lzma" | "gz" | "lzh" | "rar" | "tar" | "zst" | "xz" | "001" | "nupkg" => {
            if let Err(err) = extract_7z(archive, extract_directory) {
                if let ExtractError::SevenZNotFound = err {
                    let Ok(()) = dependency_install(config, "7zip") else {
                        return Err(ExtractError::FailedToInstall7zip);
                    };
                    return extract_archive(config, archive, extract_directory, dir_to_extract);
                }

                return Err(err);
            }
        }
        "msi" => {
            extract_msi(archive, extract_directory)?;
        }
        "exe" => {
            // if it is an exe just copy it to the extract_directory
            // ignore the returned value (number of bits copied)
            let _ = std::fs::copy(
                archive,
                extract_directory.join(archive.file_name().ok_or(ExtractError::NoFileNameError)?),
            )?;
        }
        _ => return Err(ExtractError::UnsupportedArchive(file_type.to_string())),
    };

    println!("Extracted archive successfully");

    let _ = strip_directory(extract_directory, dir_to_extract);
    remove_file(archive)?;

    Ok(())
}

fn extract_7z(archive: &Path, extract_dir: &Path) -> Result<(), ExtractError> {
    use std::process::Command;

    let output = Command::new("pwsh")
        .args([
            "-c",
            "7z",
            "x",
            archive.to_str().ok_or(ExtractError::OsStrConversionError)?,
            &format!(
                "-o{}",
                extract_dir
                    .to_str()
                    .ok_or(ExtractError::OsStrConversionError)?
            ),
            //"-xr!.nisis", figure this out
            "-y",
        ])
        .output()?;

    //let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    // println!("STDOUT: {stdout}");
    //println!("STDERR: {stderr}");

    if stderr.contains("The term '7z' is not recognized") {
        return Err(ExtractError::SevenZNotFound);
    }

    Ok(())
}

/// A swift implimentation to extract the contents of the msi file into the given directory
pub fn extract_msi(archive: &Path, extract_dir: &Path) -> Result<(), ExtractError> {
    // WARN FATAL msiexec won't extract the files into the specified directory if the msi has
    // already been installed (i.e.) registered in the windows uninstaller. we needa figure out how
    // to unregister it from there sto be able to support multiple versions for .msi files

    use std::process::Command;
    let archive = archive.to_str().ok_or(ExtractError::OsStrConversionError)?;
    let extract_dir = extract_dir
        .to_str()
        .ok_or(ExtractError::OsStrConversionError)?
        .to_owned()
        + "\\";

    println!("WARN support for msi installation is incomplete!");
    let output = Command::new("pwsh")
        .args([
            "-c",
            "msiexec.exe",
            "/i",
            archive,
            "/qn",
            &format!("INSTALLDIR={extract_dir}"),
        ])
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;

    if !stderr.is_empty() {
        return Err(ExtractError::StdErr(stderr));
    }

    Ok(())
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

    if read_dir(extract_dir)?.count() > 1 {
        return Ok(());
    }

    let lonely_entry = read_dir(extract_dir)?.last().unwrap()?;

    if lonely_entry.path().is_file() {
        return Ok(());
    }

    for subdir_entry in fs::read_dir(lonely_entry.path())? {
        let subdir_entry = subdir_entry?;
        fs::rename(
            subdir_entry.path(),
            extract_dir.join(subdir_entry.file_name()),
        )?;
    }
    fs::remove_dir(lonely_entry.path())?;
    strip_directory(extract_dir, dir_to_extract)
}
