use std::path::PathBuf;

use crate::{
    cli::subcommands::find_package,
    errors::extraction::ExtractError,
    manifest::Manifest,
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
            dependency_install(config, &dependency)?;
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

    let urls = manifest.get_url().ok_or("Failed to get url".to_string())?;

    let downloaded_archives = urls
        .clone()
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
            if let Err(e) =
                extract_archive(config, archive, extract_dir, manifest.extract_dir.as_ref())
            {
                return Err(format!("Unable to Extract Archive: {e:?}"));
            };
        }
    } else {
        for archive in downloaded_archives {
            if let Err(e) = extract_archive(
                config,
                &archive,
                &extract_dir,
                manifest.extract_dir.as_ref(),
            ) {
                return Err(format!("Unable to Extract Archive: {e:?}"));
            };
        }
    }

    if let Some(bin_attribute) = manifest.get_bin() {
        let mut bin_paths = bin_attribute.normalized_executable_directores(&extract_dir);
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

    // TODO: implement this: If any of the apps suggested for the feature are already installed,
    // the feature will be treated as 'fulfilled' and the user won't see any suggestions.
    if let Some(suggestions) = &manifest.suggest {
        println!("The installed packages sugests installing the corresponding packages for the following features");
        for (key, values) in suggestions {
            print!("\x1b[92m{key}\x1b[0m : [ ");
            for value in values.clone() {
                print!("{value} ");
            }
            println!("]");
        }
    }

    Ok(())
}

pub fn dependency_install(config: &AlephConfig, dependency: &str) -> Result<(), String> {
    let Some(manifest_path) = find_package(config, dependency) else {
        if dependency != "7zip" {
            return Err(format!("Unable to install DEPENDENCY {dependency}"));
        }

        let manifest_data = SEVENZIP_MANIFEST;
        let manifest = Manifest::parse(manifest_data).expect("Failed to parse manifest");
        println!("\x1b[92mInstalling Dependency {dependency}\x1b[0m");
        manifest_installer(config, &manifest, dependency)?;
        return Ok(());
    };
    let manifest_data = std::fs::read_to_string(manifest_path).expect("Failed to read Manifest");
    let manifest = Manifest::parse(&manifest_data).expect("Failed to parse manifest");
    println!("\x1b[92mInstalling Dependency {dependency}\x1b[0m");
    manifest_installer(config, &manifest, dependency)?;
    Ok(())
}

// ugly workaround will think of something later
const SEVENZIP_MANIFEST: &str = r#"{
    "version": "24.09",
    "description": "A multi-format file archiver with high compression ratios",
    "homepage": "https://www.7-zip.org/",
    "license": "LGPL-2.1-or-later",
    "notes": "Add 7-Zip as a context menu option by running: \"$dir\\install-context.reg\"",
    "architecture": {
        "64bit": {
            "url": "https://www.7-zip.org/a/7z2409-x64.msi",
            "hash": "ec6af1ea0367d16dde6639a89a080a524cebc4d4bedfe00ed0cac4b865a918d8",
            "extract_dir": "Files\\7-Zip"
        },
        "32bit": {
            "url": "https://www.7-zip.org/a/7z2409.msi",
            "hash": "c7f182dad21eebfce02f141d6a01f847d1e194c4d6aa29998d9305388553cf6a",
            "extract_dir": "Files\\7-Zip"
        },
        "arm64": {
            "url": "https://www.7-zip.org/a/7z2409-arm64.exe",
            "hash": "bc7b3a18f218f4916e1c4996751468f96e46eb7e97e91e8c1553d74793037f1a",
            "pre_install": [
                "$7zr = Join-Path $env:TMP '7zr.exe'",
                "Invoke-WebRequest https://www.7-zip.org/a/7zr.exe -OutFile $7zr",
                "Invoke-ExternalCommand $7zr @('x', \"$dir\\$fname\", \"-o$dir\", '-y') | Out-Null",
                "Remove-Item \"$dir\\Uninstall.exe\", \"$dir\\*-arm64.exe\", $7zr"
            ]
        }
    },
    "post_install": [
        "$7zip_root = \"$dir\".Replace('\\', '\\\\')",
        "'install-context.reg', 'uninstall-context.reg' | ForEach-Object {",
        "    $content = Get-Content \"$bucketsdir\\main\\scripts\\7-zip\\$_\"",
        "    $content = $content.Replace('$7zip_root', $7zip_root)",
        "    if ($global) {",
        "       $content = $content.Replace('HKEY_CURRENT_USER', 'HKEY_LOCAL_MACHINE')",
        "    }",
        "    Set-Content \"$dir\\$_\" $content -Encoding Ascii",
        "}"
    ],
    "bin": [
        "7z.exe",
        "7zFM.exe",
        "7zG.exe"
    ],
    "shortcuts": [
        [
            "7zFM.exe",
            "7-Zip"
        ]
    ],
    "persist": [
        "Codecs",
        "Formats"
    ],
    "checkver": {
        "url": "https://www.7-zip.org/download.html",
        "regex": "Download 7-Zip ([\\d.]+) \\(\\d{4}-\\d{2}-\\d{2}\\)"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://www.7-zip.org/a/7z$cleanVersion-x64.msi"
            },
            "32bit": {
                "url": "https://www.7-zip.org/a/7z$cleanVersion.msi"
            },
            "arm64": {
                "url": "https://www.7-zip.org/a/7z$cleanVersion-arm64.exe"
            }
        }
    }
}"#;
