mod subcommands;
use subcommands::SubCommand;

use crate::AlephConfig;
pub struct Action {
    sub_command: SubCommand,
    argument: Option<String>,
}

impl Action {
    fn new(sub_command: SubCommand, argument: Option<String>) -> Self {
        Self {
            sub_command,
            argument,
        }
    }

    pub fn parse(env_args: std::env::Args) -> Result<Self, ()> {
        if env_args.len() == 1 {
            return Ok(Action::new(SubCommand::Help, None));
        }

        // the first index(0) contains the full path of the program
        // the second index(1) contains our primary argument [sub command]
        // then third onwards we have the arguments passed to the sub command
        let mut primary_argument = None;
        let mut support_arguments = String::from("");

        for argument in env_args.skip(1) {
            if primary_argument.is_none() {
                primary_argument = Some(argument);
            } else {
                support_arguments += &(argument + " ");
            }
        }

        let primary_argument = primary_argument.unwrap();
        let support_arguments = if support_arguments.is_empty() {
            None
        } else {
            Some(support_arguments)
        };

        match primary_argument.as_str() {
            "--help" => Ok(Action::new(SubCommand::Help, None)),
            "install" => Ok(Action::new(SubCommand::Install, support_arguments)),
            "fetch" => Ok(Action::new(SubCommand::Fetch, support_arguments)),
            "search" => Ok(Action::new(SubCommand::Search, support_arguments)),
            "uninstall" => Ok(Action::new(SubCommand::Uninstall, support_arguments)),
            _ => Err(()),
        }
    }

    pub fn dispatch(&self, config: &AlephConfig) -> Result<(), String> {
        self.sub_command.dispatch(config, self.argument.as_ref())
    }
}
