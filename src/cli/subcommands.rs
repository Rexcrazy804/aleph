pub(super) enum SubCommand {
    // help is a special subcommand for the --help flag
    Help,
    Search,
    Install,
    Fetch,

    // future
    Rebuild,
}

impl SubCommand {
    pub fn dispatch(&self, argument: Option<&String>) -> Result<(), String> {
        match self {
            SubCommand::Help => Ok(display_help()),
            SubCommand::Search => todo!(),
            SubCommand::Install => todo!(),
            SubCommand::Fetch => todo!(),
            SubCommand::Rebuild => todo!(),
        }
    }
}

/// Function that breifly introduces all the subcommands
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
