use crate::{
    manifest::{bin::Binary, Manifest, OneOrMany},
    powershell::{
        installer::append_to_path,
        utilities::{download_url, get_home_directory},
    },
    zipper::unzip_alt,
};

// DEBUG SYSMBOLS
// all must be set to FALSE after debugging
const DEBUG_NOINSTALL: bool = false;
const DEBUG_PRINT: bool = false;

// TODO Replace error return type to a concrete enum that can account for the different errors
// no sanoy this is not for you
pub fn manifest_installer(manifest: &Manifest) -> Result<(), String> {
    let home_dir = get_home_directory();
    let download_dir = format!("{home_dir}\\Downloads\\");
    let extract_dir = format!("{home_dir}\\Documents\\aleph\\");

    let url = manifest.get_url();

    // TODO hash the downloads so that we can extract them
    // into <hash>-filename/ directories like nixos
    let file_path = match url {
        OneOrMany::One(url) => OneOrMany::One(download_url(&url, &download_dir)),
        OneOrMany::Many(urls) => OneOrMany::Many(
            urls.iter()
                .map(|url| download_url(url, &download_dir))
                .collect(),
        ),
    };

    if DEBUG_PRINT {
        println!("{file_path:?}");
    }

    let extracted_dir = match file_path {
        OneOrMany::One(file_path) => {
            let mut manifest_extract_dir = None;
            if let Some(OneOrMany::One(dir)) = &manifest.extract_dir {
                manifest_extract_dir = Some(dir);
            };

            let Ok(file_path) = file_path else {
                panic!("FAILIURE");
            };

            let result = unzip_alt(&file_path, &extract_dir, manifest_extract_dir);
            OneOrMany::One(result)
        }

        OneOrMany::Many(file_paths) => {
            let result;
            if let Some(OneOrMany::Many(dirs)) = &manifest.extract_dir {
                result = file_paths
                    .iter()
                    .zip(dirs)
                    .map(|(file_path, m_extract_dir)| {
                        let Ok(file_path) = file_path else {
                            panic!("FAILIURE");
                        };

                        unzip_alt(file_path, &extract_dir, Some(m_extract_dir))
                    })
                    .collect();
            } else {
                result = file_paths
                    .iter()
                    .map(|file_path| {
                        let Ok(file_path) = file_path else {
                            panic!("FAILIURE");
                        };

                        unzip_alt(file_path, &extract_dir, None)
                    })
                    .collect()
            };
            OneOrMany::Many(result)
        }
    };

    if DEBUG_PRINT {
        println!("EXTRACTED DIRECTORY: {extracted_dir:?}");
    }

    // currently there is no real need to do this but once we start hashing our downloads we might
    // end up
    // WARN kinda hit a road bloack with shims, will leave it here for the time being
    // TODO do something about shims :')

    // NOTE for the time being we'll be using this
    let _ = match extracted_dir {
        OneOrMany::One(dir) => {
            let exutables =
                get_executables(&dir, manifest.bin.clone().expect("No Binary found")).unwrap();
            if !DEBUG_NOINSTALL {
                append_to_path(&home_dir, &exutables).expect("failed to append to path")
            }
        }
        OneOrMany::Many(dirs) => {
            for dir in dirs {
                let exutables =
                    get_executables(&dir, manifest.bin.clone().expect("No Binary found")).unwrap();

                if !DEBUG_NOINSTALL {
                    append_to_path(&home_dir, &exutables).expect("Failed to append to path")
                }
            }
        }
    };

    Ok(())
}

fn get_executables(search_dir: &str, bin_attr: Binary) -> Result<Vec<String>, String> {
    let mut binary_paths: Vec<String> = Vec::new();

    if let Binary::Executable(exe) = &bin_attr {
        binary_paths.append(&mut find_binaries(search_dir, &[exe.to_string()]));
    };

    if let Binary::Executables(exes) = &bin_attr {
        binary_paths.append(&mut find_binaries(search_dir, exes));
    };

    if let Binary::AliasedExecutables(aliased_exes) = &bin_attr {
        for alias_or_exe in aliased_exes {
            if let Binary::Executable(exe) = &alias_or_exe {
                binary_paths.append(&mut find_binaries(search_dir, &[exe.to_string()]));
            };

            // now here comes the hard part :')
            #[allow(unused_variables)]
            if let Binary::Executables(aliases) = alias_or_exe {
                // WARN TODO
                // for the the time being we  will do nothing; plan is to create a function that
                // can handle this that will hopefully create a new aliased executable into the
                // ${search_directory} which will work as aliases are intended to work i.e being a
                // plain alias to a direct exe call OR calling the exe with specified parametres
            };
        }
    };
    Ok(binary_paths)
}

/// returns the full path of each binary listed in the bin attribute of the manifest
fn find_binaries(search_dir: &str, binaries: &[String]) -> Vec<String> {
    // my question is whether we should directly add the binary itself to the PATH variable or whether we
    // just need to include the parent directory?
    // lets find out I suppose
    // I found out and I am not happy with the result
    let mut path_to_binary_parent_dirs: Vec<String> = Vec::new();
    path_to_binary_parent_dirs.push(search_dir.to_string());

    for binary in binaries {
        let parrent_dir = binary.split('\\').rev().skip(1).collect::<String>();
        if parrent_dir.is_empty() {
            continue;
        }

        let full_path_to_parent = format!("{search_dir}\\{parrent_dir}");
        path_to_binary_parent_dirs.push(full_path_to_parent);

        if DEBUG_PRINT {
            println!("binary: {binary}");
            println!("parent dir: {parrent_dir}");
        }
    }

    path_to_binary_parent_dirs.sort();
    path_to_binary_parent_dirs.dedup();

    if DEBUG_PRINT {
        println!("parent paths: {path_to_binary_parent_dirs:?}");
    }

    path_to_binary_parent_dirs
}
