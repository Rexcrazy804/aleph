use std::any::Any;

use crate::manifest::Manifest;
use crate::AlephConfig;
use std::path::Path;

pub(super) enum SubCommand {
    // help is a special subcommand for the --help flag
    Help,
    Search,
    Install,
    Fetch,

    // future [eta end of march]
    #[allow(dead_code)]
    Rebuild,
}

impl SubCommand {
    pub fn dispatch(&self, config: &AlephConfig, argument: Option<&String>) -> Result<(), String> {
        match self {
            SubCommand::Help => {
                display_help();
                Ok(())
            }
            SubCommand::Search => search_repo(config, argument),
            SubCommand::Install => install_repo_manifest(config, argument),
            SubCommand::Fetch => fetch_repo(config, argument),
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

fn fetch_repo(config: &AlephConfig, argument: Option<&String>) -> Result<(), String> {
    use crate::powershell::utilities::download_url;
    use crate::zipper::extract_archive;
    use std::fs::{create_dir, rename};

    let (bucket_name, url) = if let Some(arg) = argument {
        let arg = arg.trim();
        arg.split_once(' ').expect("Invalid arguments to fetch")
    } else {
        println!("WARN NO ARGUMENT PROVIDED, assuming default bucket main");
        (
            "main",
            "https://github.com/ScoopInstaller/Main/archive/refs/heads/master.zip",
        )
    };

    let bucket_dir = config.paths.buckets.join(bucket_name);
    if let Ok(true) = bucket_dir.try_exists() {
        println!("Bucket already exists / use a different bucket name!!");
        return Ok(());
    }

    create_dir(&bucket_dir).expect("Failed to create directory for bucket {bucket_name}");

    let Ok(archive) = download_url(url, &config.paths.download, &config.paths.packages) else {
        return Err("Failed to download File".to_string());
    };

    if archive.extension().is_some() {
        extract_archive(&archive, &bucket_dir, None);
    } else {
        // in the event that the provided bucket does not have a file extension, we will assume
        // that it is a zip file.
        let mut new_archive = archive.clone();
        new_archive.set_file_name("bucket.zip");
        rename(archive, &new_archive).expect("Failed to rename archive");
        extract_archive(&new_archive, &bucket_dir, None);
    }

    Ok(())
}

fn install_repo_manifest(config: &AlephConfig, pname: Option<&String>) -> Result<(), String> {
    use crate::manifest::Manifest;
    use crate::powershell::utilities::get_home_directory;
    use crate::scoopd::manifest_install::manifest_installer;
    use std::fs::read_to_string;

    let Some(pname) = pname else {
        return Err("package name REQUIRED".to_string());
    };

    let home_dir = get_home_directory();
    // will need to modify this when multi bucket support is added
    let repo_dir = home_dir.join("Documents\\aleph\\__REPO-masterfile\\bucket");

    for package in pname.split_whitespace() {
        // lets us do stuff like aleph install p1 p2 p3 p4
        let manifest_path = repo_dir.join(format!("{package}.json"));

        let manifest =
            read_to_string(manifest_path).expect("Failed to find manifest. Invalid package name?");
        let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
        manifest_installer(config, &manifest, pname)?;
    }

    Ok(())
}

fn search_repo(config: &AlephConfig, keywords: Option<&String>) -> Result<(), String> {
    // will need to modify this when multi bucket support is added
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

fn search_bucket(keywords: &Vec<&str>, bucket: &Path) {
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
