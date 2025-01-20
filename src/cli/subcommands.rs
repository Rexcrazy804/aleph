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
    pub fn dispatch(&self, argument: Option<&String>) -> Result<(), String> {
        match self {
            SubCommand::Help => Ok(display_help()),
            SubCommand::Search => todo!(),
            SubCommand::Install => install_repo_manifest(argument),
            SubCommand::Fetch => fetch_repo(argument),
            SubCommand::Rebuild => todo!(),
        }
    }
}

/// breifly introduces all the subcommands
fn display_help() {
    println!("aleph <subcommand> <argument>");
    colorize_print_description(
        "92",
        "search",
        "search for packages in the current repository",
    );
    colorize_print_description(
        "92",
        "install",
        "install packages in the current repository",
    );
    colorize_print_description(
        "92",
        "fetch",
        "fetch the latest available version of the repository",
    );
    colorize_print_description("92", "--help", "display this help");
}

fn colorize_print_description(color: &str, command: &str, description: &str) {
    // whacky way of doing it for the time being
    // TODO improve the tabs to be dynamic [something based off the longest command .w.]
    let tabs = if command.len() > 6 { "\t\t" } else { "\t\t\t" };
    println!("\x1b[{color}m{command}\x1b[0m{tabs}- {description}");
}

fn fetch_repo(url: Option<&String>) -> Result<(), String> {
    use crate::powershell::utilities::{download_url, get_home_directory};
    use crate::zipper::unzip_alt;

    let url = if url.is_some() {
        url.unwrap()
    } else {
        "https://codeload.github.com/ScoopInstaller/Main/zip/refs/heads/master"
    };

    // prolly have to condense this into a config that is readable
    // maybe
    let home_dir = get_home_directory();
    let download_dir = dbg!(format!("{home_dir}\\Downloads\\"));
    let extract_dir = dbg!(format!(
        "{home_dir}\\Documents\\aleph\\__REPO-{}",
        url.split('/')
            .last()
            .expect("Failed to identify bucket name")
    ));

    let Ok(file_path) = download_url(url, &download_dir) else {
        return Err("Failed to download File".to_string());
    };

    // we aren't using the path for the time being but we will need to log it down somwhere
    // once support for mutliple repos are established
    let _ = unzip_alt(&file_path, &extract_dir, None);
    Ok(())
}

fn install_repo_manifest(pname: Option<&String>) -> Result<(), String> {
    use crate::manifest::Manifest;
    use crate::powershell::utilities::get_home_directory;
    use crate::scoopd::manifest_install::manifest_installer;
    use std::fs::read_to_string;

    let Some(pname) = pname else {
        return Err("package name REQUIRED".to_string());
    };

    let home_dir = get_home_directory();
    let repo_dir = dbg!(format!(
        "{home_dir}\\Documents\\aleph\\__REPO-masterfile\\bucket"
    ));

    for package in pname.split_whitespace() {
        // lets us do stuff like aleph install p1 p2 p3 p4
        let manifest_path = dbg!(format!("{repo_dir}\\{package}.json"));

        let manifest =
            read_to_string(manifest_path).expect("Failed to find manifest. Invalid package name?");
        let manifest: Manifest = serde_json::from_str(&manifest).expect("Failed to parse data");
        manifest_installer(&manifest)?
    }

    Ok(())
}
