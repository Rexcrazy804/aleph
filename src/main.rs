use aleph::{cli::Action, AlephConfig};
use std::env;

fn main() {
    #[cfg(target_os = "linux")]
    panic!("Invalid Platform");

    // advertizes the os the program is running on
    println!(
        "Running on {} arch {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    let config = AlephConfig::new();

    let action = Action::parse(env::args())
        .expect("Invalid Subcommand. Use aleph --help for a list of subcommands");

    // TODO: Error handling for this
    if let Err(error) = action.dispatch(&config) {
        eprintln!("{error}");
    };
}
