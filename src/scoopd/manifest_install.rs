use std::path::PathBuf;

use crate::{
    cli::subcommands::find_package,
    manifest::{bin::normalized_executable_directores, Manifest},
    powershell::{installer::append_to_path, utilities::download_url},
    zipper::extract_archive,
    AlephConfig,
};

// TODO Replace error return type to a concrete enum that can account for the different errors
// no sanoy this is not for you
/// # Errors
/// TODO: populate [document the possible erros sanoy you can do this part]
/// # Panics
/// Failing to read or parse dependencies
/// TODO: populate [document the panic sanoy you can do this too]
pub fn manifest_installer(
    config: &AlephConfig,
    manifest: &Manifest,
    package_name: &str,
) -> Result<(), String> {
    if let Some(dependencies) = &manifest.depends {
        for dependency in dependencies.clone() {
            let Some(manifest_path) = find_package(config, &dependency) else {
                return Err(format!("Unable to install DEPENDENCY {dependency}"));
            };

            let manifest_data =
                std::fs::read_to_string(manifest_path).expect("Failed to read Manifest");
            let manifest = Manifest::parse(&manifest_data).expect("Failed to parse manifest");
            println!("\x1b[92mInstalling Dependency {dependency}\x1b[0m");
            manifest_installer(config, &manifest, &dependency)?;
        }
    };

    // NOTE: if two buckets have packages with the same package name WE MUST force the user to
    // declare which bucket the package is to be downloaded from. The user may declare the package
    // to be installed from both buckets in which case we will need to set package name as
    // package_name = <bucket-name>-<Package-name>
    // TODO: implement above funtionality.
    // Files will be installed to ROOT_DIR/Packages/<Package-name>/<Package_version>/
    if let Some(notes) = manifest.notes.as_ref() {
        for note in notes.clone() {
            println!("\x1b[92m{note}\x1b[0m");
        }
    }

    let package_version = &manifest.version;
    let extract_dir = config
        .paths
        .packages
        .join(package_name)
        .join(package_version);

    // TODO (sanoy) check if program exists in path as well before exiting
    // if let Ok(true) = extract_dir.try_exists() {
    //    println!("Program {package_name} version {package_version} has already been installed");
    //    return Ok(())
    //}

    let downloaded_archives = manifest
        .get_url()
        .map(
            |url| match download_url(&url, &config.paths.download, &config.paths.packages) {
                Ok(dir) => dir,
                Err(e) => panic!("{e}"),
            },
        )
        .collect::<Vec<PathBuf>>();

    if let Some(extract_to_paths) = manifest.extract_to.as_ref() {
        for (archive, extract_to_path) in downloaded_archives.iter().zip(extract_to_paths.clone()) {
            let dont_change_path = extract_to_path.is_empty() || extract_to_path == ".";
            let extract_dir = if dont_change_path {
                &extract_dir
            } else {
                &extract_dir.join(extract_to_path)
            };
            extract_archive(archive, extract_dir, manifest.extract_dir.as_ref());
        }
    } else {
        for archive in downloaded_archives {
            extract_archive(&archive, &extract_dir, manifest.extract_dir.as_ref());
        }
    }

    if let Some(bin_attribute) = manifest.get_bin() {
        let mut bin_paths = normalized_executable_directores(&extract_dir, bin_attribute);
        if bin_paths.is_empty() {
            let _ = append_to_path(&config.paths.home, &vec![extract_dir]);
        } else {
            bin_paths.sort();
            bin_paths.dedup();

            let _ = append_to_path(&config.paths.home, &bin_paths);
        }
    } else {
        let _ = append_to_path(&config.paths.home, &vec![extract_dir]);
    }

    println!("\x1b[92minstalled {package_name}\x1b[0m");
    Ok(())
}
