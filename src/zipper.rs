use crate::errors::extraction::ExtractError;
use crate::scoopd::manifest_install::dependency_install;
use crate::AlephConfig;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

/// unzips ``file_path`` to ``package_dir``
/// if ``extract_dir`` is provided, then only that directory is extracted out of the ``archive``
/// # Errors
/// - most important error being failure to find 7zip in path
/// - rest are well described in the ``ExtractError`` definition
pub fn extract_archive(
    config: &AlephConfig,
    archive: &Path,
    package_dir: &Path,
    extract_dir: Option<&str>,
) -> Result<(), ExtractError> {
    // I am certain we can skip the make dirname step by reading and tracking the name of the .json
    // file and then modify the function to extract directly to package_dir instead of making
    // a folder on top of etract dir
    // TODO: add optional argument to explicitly provide filename
    use fs::{create_dir_all, remove_file};
    let file_type = archive
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(ExtractError::NoFileExtensionError)?;

    create_dir_all(package_dir)?;
    println!("Created package directory");

    match file_type {
        "7z" | "zip" | "lzma" | "gz" | "lzh" | "rar" | "tar" | "zst" | "xz" | "001" | "nupkg" => {
            if let Err(err) = extract_7z(archive, package_dir, extract_dir) {
                if let ExtractError::SevenZNotFound = err {
                    let Ok(()) = dependency_install(config, "7zip") else {
                        return Err(ExtractError::FailedToInstall7zip);
                    };
                    return extract_archive(config, archive, package_dir, extract_dir);
                }

                return Err(err);
            }
        }
        "msi" => {
            extract_msi(archive, package_dir)?;
        }
        "exe" => {
            // if it is an exe just copy it to the package_dir
            // ignore the returned value (number of bits copied)
            let _ = std::fs::copy(
                archive,
                package_dir.join(archive.file_name().ok_or(ExtractError::NoFileNameError)?),
            )?;
        }
        _ => return Err(ExtractError::UnsupportedArchive(file_type.to_string())),
    };

    println!("Extracted archive successfully");

    strip_directory(package_dir)?;
    remove_file(archive)?;

    Ok(())
}

fn extract_7z(
    archive: &Path,
    package_dir: &Path,
    extract_dir: Option<&str>,
) -> Result<(), ExtractError> {
    use std::process::Command;

    let extract_dir = if let Some(extract_dir) = extract_dir {
        format!("-ir!{extract_dir}\\*")
    } else {
        String::new()
    };

    let output = Command::new("pwsh")
        .args([
            "-c",
            "7z",
            "x",
            archive.to_str().ok_or(ExtractError::OsStrConversionError)?,
            &format!(
                "-o{}",
                package_dir
                    .to_str()
                    .ok_or(ExtractError::OsStrConversionError)?
            ),
            //"-xr!.nisis", figure this out
            &extract_dir,
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
pub fn extract_msi(archive: &Path, package_dir: &Path) -> Result<(), ExtractError> {
    // WARN FATAL msiexec won't extract the files into the specified directory if the msi has
    // already been installed (i.e.) registered in the windows uninstaller. we needa figure out how
    // to unregister it from there sto be able to support multiple versions for .msi files

    use std::process::Command;
    let archive = archive.to_str().ok_or(ExtractError::OsStrConversionError)?;
    let package_dir = package_dir
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
            &format!("INSTALLDIR={package_dir}"),
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
/// ``dir_to_extract`` comes from ``Manifest.package_dir`` and is not used as of now
fn strip_directory(package_dir: &Path) -> Result<(), ExtractError> {
    use fs::read_dir;

    if read_dir(package_dir)?.count() > 1 {
        return Ok(());
    }

    let Some(lonely_entry) = read_dir(package_dir)?.last() else {
        return Err(ExtractError::FailedToExtract);
    };

    let lonely_entry = lonely_entry?;

    if lonely_entry.path().is_file() {
        return Ok(());
    }

    for subdir_entry in fs::read_dir(lonely_entry.path())? {
        let subdir_entry = subdir_entry?;
        fs::rename(
            subdir_entry.path(),
            package_dir.join(subdir_entry.file_name()),
        )?;
    }
    fs::remove_dir(lonely_entry.path())?;
    strip_directory(package_dir)
}
