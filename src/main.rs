use aleph::cli::Action;
use std::env;

fn main() {
    // advertizes the os the program is running on

    #[cfg(target_os = "linux")]
    panic!("Invalid Platform");

    println!(
        "Running on {} arch {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    // The main driver progrem is incomplete
    // use: cargo test --test install_test -- --ignored --show-output
    // to run the install test that currenlty drives the installer

    // [immediate]
    // aleph search <search string>             # same as below
    // aleph install <package name>             # if repo not found at ~/Documents/Aleph/__REPO call aleph fetch
    // aleph fetch [opt: <url>]                  # fetches the latest commit from the scoop branch
    // aleph --help                             # displays all the available sub commands
    // aleph <subcommand> --help                # usage information for the subcommand
    // aleph                                    # should just run aleph --help
    //
    // [FUTURE]
    // aleph rebuild <config file>

    let action = Action::parse(env::args())
        .expect("Invalid Subcommand. Use aleph --help for a list of subcommands");

    // TODO: Error handling for this
    let _ = action.dispatch();
}
