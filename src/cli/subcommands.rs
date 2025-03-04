use crate::AlephConfig;
use crate::{manifest::Manifest, scoopd::manifest_uninstall::manifest_uninstaller};
use std::path::{Path, PathBuf};

pub enum SubCommand {
    // help is a special subcommand for the --help flag
    Help,
    Search,
    Install,
    Fetch,
    Uninstall,

    // future [eta end of march]
    #[allow(dead_code)]
    Rebuild,
}

impl SubCommand {
    /// simple dispatcher for available subcommands
    /// # Errors
    /// Returns relevant errors as a string .w. [we can do better]
    pub fn dispatch(&self, config: &AlephConfig, arguments: Option<&String>) -> Result<(), String> {
        match self {
            SubCommand::Help => {
                display_help();
                Ok(())
            }
            SubCommand::Search => search_repo(config, arguments),
            SubCommand::Install => install_repo_manifest(config, arguments),
            SubCommand::Fetch => fetch_repo(config, arguments),
            SubCommand::Uninstall => uninstall_package(config, arguments),
            SubCommand::Rebuild => unimplemented!(""),
        }
    }
}

/// breifly introduces all the subcommands
fn display_help() {
    println!("aleph <subcommand> <argument>");
    colorize_print_description(
        "92",
        "search <package>",
        "search for packages in the current repository",
        None,
    );
    colorize_print_description(
        "92",
        "install <package>",
        "install packages in the current repository",
        None,
    );
    colorize_print_description(
        "92",
        "fetch <bucketname> <url>",
        "fetch a given bucket from url",
        None,
    );
    colorize_print_description("92", "--help", "display this help", None);
}

fn colorize_print_description(color: &str, command: &str, description: &str, tabs: Option<&str>) {
    // whacky way of doing it for the time being
    // TODO improve the tabs to be dynamic [something based off the longest command .w.]
    let tabs = tabs.unwrap_or(if command.len() > 6 { "\t\t" } else { "\t\t\t" });
    println!("\x1b[{color}m{command}\x1b[0m{tabs}- {description}");
}

pub fn fetch_repo(config: &AlephConfig, args: Option<&String>) -> Result<(), String> {
    use crate::powershell::utilities::download_url;
    use crate::zipper::extract_archive;
    use std::fs::{create_dir, rename};

    let Some(args) = args else {
        println!("WARN NO ARGUMENT PROVIDED, installing default buckets main, extras");
        let main_bucket =
            "main https://github.com/ScoopInstaller/Main/archive/refs/heads/master.zip";
        let extras_bucket =
            "extras https://github.com/ScoopInstaller/Extras/archive/refs/heads/master.zip";
        fetch_repo(config, Some(&main_bucket.to_string()))?;
        fetch_repo(config, Some(&extras_bucket.to_string()))?;
        return Ok(());
    };

    let Some((bucket_name, url)) = args.split_once(' ') else {
        return Err("Invalid arguments".to_string());
    };

    let bucket_dir = config.paths.buckets.join(bucket_name);
    if let Ok(true) = bucket_dir.try_exists() {
        println!("Bucket already exists / use a different bucket name!!");
        return Ok(());
    }

    create_dir(&bucket_dir).map_err(|e| e.to_string())?;

    let Ok(archive) = download_url(url, &config.paths.download, &config.paths.packages) else {
        return Err("Failed to download File".to_string());
    };

    if archive.extension().is_some() {
        extract_archive(config, &archive, &bucket_dir, None).map_err(|e| e.to_string())?;
    } else {
        // in the event that the provided bucket does not have a file extension, we will assume
        // that it is a zip file.
        let mut new_archive = archive.clone();
        new_archive.set_file_name("bucket.zip");
        rename(archive, &new_archive).map_err(|e| e.to_string())?;
        extract_archive(config, &new_archive, &bucket_dir, None).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn install_repo_manifest(config: &AlephConfig, args: Option<&String>) -> Result<(), String> {
    use crate::manifest::Manifest;
    use crate::scoopd::manifest_install::manifest_installer;
    use std::fs::read_to_string;

    let Some(args) = args else {
        return Err("package name REQUIRED".to_string());
    };

    let packages = args.split_whitespace();
    for package in packages {
        let package = package.trim();

        let Some(manifest_path) = find_package(config, package) else {
            println!("Pakcage {package} not found");
            continue;
        };

        let manifest_data =
            read_to_string(manifest_path).expect("Failed to find manifest. Invalid package name?");
        let manifest = Manifest::parse(&manifest_data).expect("Failed to parse data");
        manifest_installer(config, &manifest, package)?;
    }

    Ok(())
}

pub fn uninstall_package(config: &AlephConfig, args: Option<&String>) -> Result<(), String> {
    // Ensure we have an argument.
    let arg = args.ok_or("Package name required for uninstall.".to_string())?;

    // Split the argument string into individual package names.
    for pkg in arg.split_whitespace().map(str::trim) {
        // Compute the expected package directory: $HOME\Aleph\Packages\<pkg>
        let package_path: PathBuf = config.paths.packages.join(pkg);
        if !package_path.exists() {
            println!("Package '{pkg}' not found at {package_path:?}");
            continue; // Skip this package if the directory does not exist.
        }
        println!("Found package '{pkg}' at {package_path:?}. Proceeding with uninstall...");

        // Call the manifest uninstallation logic.
        // We assume that in your manifest_uninstall.rs you have a function like:
        // `pub fn uninstall_repo_manifest(config: &AlephConfig, arg: Option<&String>) -> Result<(), String>`
        // which handles deleting the files for a given package.
        manifest_uninstaller(config, pkg)?;
    }
    Ok(())
}

pub(crate) fn find_package(config: &AlephConfig, package: &str) -> Option<PathBuf> {
    for bucket in config.paths.buckets.read_dir().expect("") {
        let Ok(bucket) = bucket else {
            println!("Failed to read bucket entry");
            continue;
        };
        let manifest_bucket_path = bucket.path().join("bucket").join(format!("{package}.json"));
        if let Ok(true) = manifest_bucket_path.try_exists() {
            println!("found package manifest at {manifest_bucket_path:?}");
            return Some(manifest_bucket_path);
        }
    }

    None
}

fn search_repo(config: &AlephConfig, keywords: Option<&String>) -> Result<(), String> {
    let buckets_path = &config.paths.buckets;

    let Some(keywords) = keywords else {
        return Err("Expected keyword argument for search subcommand".to_string());
    };

    let keywords = keywords.split_whitespace().collect::<Vec<&str>>();

    for bucket in std::fs::read_dir(buckets_path).expect("Failed to read Directory") {
        let bucket = bucket.expect("failed to read entry").path();
        // we are assuming that every entry here is a directory, skip if it isn't
        if bucket.is_file() {
            continue;
        }

        let bucket = bucket.join("bucket");
        search_bucket(&keywords, &bucket);
    }

    Ok(())
}

//TODO add optional support for specifying bucket
/// searches for the given package name within all available buckets and returns the first matching
/// entry
///
/// # Panics
/// Will panic the buckets directory does not exist / is inaccesible
pub fn search_bucket(keywords: &Vec<&str>, bucket: &Path) {
    for manifest_file in bucket.read_dir().expect("Failed to read dir") {
        let Ok(manifest_file) = manifest_file else {
            println!("Failed to read entry: {manifest_file:?}");
            continue;
        };

        let manifest_file = manifest_file.path();
        let manifest_name = manifest_file
            .file_name()
            .expect("No file name?")
            .to_str()
            .expect("Failed to convert file name to string");

        for word in keywords {
            if !manifest_name.contains(word) {
                continue;
            }

            let Ok(manifest_data) = std::fs::read_to_string(&manifest_file) else {
                println!("Failed to read {manifest_name:?}");
                break;
            };

            let manifest = Manifest::parse(&manifest_data).expect("Failed to parse manifest");
            colorize_print_description("92", manifest_name, &manifest.description, Some("\t"));

            break;
        }
    }
}
