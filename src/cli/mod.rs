mod subcommands;
use subcommands::SubCommand;
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

        let mut env_args = env_args;
        let primary_argument = env_args.nth(1).unwrap();
        let support_arguments: String = env_args.collect();
        let support_arguments = if support_arguments.is_empty() {
            None
        } else {
            Some(support_arguments)
        };

        dbg!(&support_arguments);
        match primary_argument.as_str() {
            "--help" => Ok(Action::new(SubCommand::Help, None)),
            "install" => Ok(Action::new(SubCommand::Install, support_arguments)),
            "fetch" => Ok(Action::new(SubCommand::Fetch, support_arguments)),
            "search" => Ok(Action::new(SubCommand::Search, support_arguments)),
            _ => Err(()),
        }
    }

    pub fn dispatch(&self) -> Result<(), String> {
        self.sub_command.dispatch(self.argument.as_ref())
    }
}
