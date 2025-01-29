use std::path::PathBuf;

use crate::{
    manifest::Manifest,
    powershell::{
        installer::append_to_path,
        utilities::{download_url, get_home_directory},
    },
    zipper::extract_archive,
};

// TODO Replace error return type to a concrete enum that can account for the different errors
// no sanoy this is not for you
/// # Errors
/// TODO: populate [document the possible erros sanoy you can do this part]
/// # Panics
/// TODO: populate [document the panic sanoy you can do this too]
pub fn manifest_installer(manifest: &Manifest, package_name: &str) -> Result<(), String> {
    let home_dir = get_home_directory();
    let root_dir = home_dir.join("Aleph");
    let download_dir = root_dir.join("Downloads");
    let extract_dir = root_dir.join("Packages");

    initialize_root_dir(&root_dir);

    let downloaded_archives = manifest
        .get_url()
        .map(|url| match download_url(&url, &download_dir) {
            Ok(dir) => dir,
            Err(e) => panic!("{e}"),
        })
        .collect::<Vec<PathBuf>>();

    // NOTE: if two buckets have packages with the same package name WE MUST force the user to
    // declare which bucket the package is to be downloaded from. The user may declare the package
    // to be installed from both buckets in which case we will need to set package name as
    // package_name = <bucket-name>-<Package-name>
    // TODO: implement above funtionality.
    // Files will be installed to ROOT_DIR/Packages/<Package-name>/<Package_version>/
    let package_version = &manifest.version;
    let extract_dir = extract_dir.join(package_name).join(package_version);

    for archive in downloaded_archives {
        extract_archive(&archive, &extract_dir, manifest.extract_dir.as_ref());
    }

    // currently there is no real need to do this but once we start hashing our downloads we might
    // end up
    // WARN kinda hit a road bloack with shims, will leave it here for the time being
    // TODO do something about shims :')

    // NOTE for the time being we'll be using this
    // We will need to go through the contents of the bin attribute to determine the nested
    // folders that need to be symlinked
    let _ = append_to_path(&home_dir, &vec![extract_dir]);

    Ok(())
}

/// this function creates the aleph root directory and popluates it with the required directory
/// skips directory creation if it exists
// unsure whether this function should have a return type
// since if anything fails here the programs stops execution
// so if this function executes successfully it can be assumed
fn initialize_root_dir(root_path: &PathBuf) {
    use std::fs::create_dir;

    if let Ok(false) = root_path.try_exists() {
        println!("Aleph root not found");
        create_dir(root_path.clone()).expect("Failed to create Aleph Root directory");
        println!("Created aleph root at {root_path:?}");
    }

    let buckets_path = root_path.join("Buckets");
    let downloads_path = root_path.join("Downloads");
    let packages_path = root_path.join("Packages");

    if let Ok(false) = buckets_path.try_exists() {
        create_dir(buckets_path).expect("Failed to create Aleph/Buckets");
    }
    if let Ok(false) = downloads_path.try_exists() {
        create_dir(downloads_path).expect("Failed to create Aleph/Downloads");
    }
    if let Ok(false) = packages_path.try_exists() {
        create_dir(packages_path).expect("Failed to create Aleph/Packages");
    }

    // // // // // // // RIP TO MAKING SIMLINKS :D THEY DO NOT WORK UNDER WINE // // // // // // //
    //// the current directory here is the Aleph/Current that will be holding all the symlinks to
    //// active packages' executables thus we only need to link this to Path
    //// extracting this into a variable so that it can be used later to include Aleph/Current into
    //// powershell path
    //let current_dir = root_path.join("Current");
    //create_dir(current_dir).expect("Failed to create Aleph/Current/");
    //// TODO: add current_dir to path (copy the append to path function pretty much);
    //
    //// symlink $HOME/Aleph to $HOME/Documents/.Aleph so that linux users can easily access
    //// aleph root directory (as wine symlinks ~/Documents to $HOME/Documents where $HOME is the
    //// wine prefix drive C's user home directory)
    //let symlink_path = symlink_path.join(".Aleph");
    //dbg!(&symlink_path); // // // // // // // // // // // // // // // // // // // // // // // //
}

// UHH work on this crap later
//fn get_executables(search_dir: &str, bin_attr: Binary) -> Result<Vec<String>, String> {
//    let mut binary_paths: Vec<String> = Vec::new();
//
//    if let Binary::Executable(exe) = &bin_attr {
//        binary_paths.append(&mut find_binaries(search_dir, &[exe.to_string()]));
//    };
//
//    if let Binary::Executables(exes) = &bin_attr {
//        binary_paths.append(&mut find_binaries(search_dir, exes));
//    };
//
//    if let Binary::AliasedExecutables(aliased_exes) = &bin_attr {
//        for alias_or_exe in aliased_exes {
//            if let Binary::Executable(exe) = &alias_or_exe {
//                binary_paths.append(&mut find_binaries(search_dir, &[exe.to_string()]));
//            };
//
//            // now here comes the hard part :')
//            #[allow(unused_variables)]
//            if let Binary::Executables(aliases) = alias_or_exe {
//                // WARN TODO
//                // for the the time being we  will do nothing; plan is to create a function that
//                // can handle this that will hopefully create a new aliased executable into the
//                // ${search_directory} which will work as aliases are intended to work i.e being a
//                // plain alias to a direct exe call OR calling the exe with specified parametres
//            };
//        }
//    };
//    Ok(binary_paths)
//}

// returns the full path of each binary listed in the bin attribute of the manifest
//fn find_binaries(search_dir: &str, binaries: &[String]) -> Vec<String> {
//    // my question is whether we should directly add the binary itself to the PATH variable or whether we
//    // just need to include the parent directory?
//    // lets find out I suppose
//    // I found out and I am not happy with the result
//    let mut path_to_binary_parent_dirs: Vec<String> = Vec::new();
//    path_to_binary_parent_dirs.push(search_dir.to_string());
//
//    for binary in binaries {
//        let parrent_dir = binary.split('\\').rev().skip(1).collect::<String>();
//        if parrent_dir.is_empty() {
//            continue;
//        }
//
//        let full_path_to_parent = format!("{search_dir}\\{parrent_dir}");
//        path_to_binary_parent_dirs.push(full_path_to_parent);
//
//    }
//
//    path_to_binary_parent_dirs.sort();
//    path_to_binary_parent_dirs.dedup();
//    path_to_binary_parent_dirs
//}
