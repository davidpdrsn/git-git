use clap::ArgMatches;
use command_chain::*;
use commands::*;
use git::Git;

pub fn run_on_staging(args: &ArgMatches) {
    OnStagingArgs::parse_args_and_run_command(&args, on_staging_command);
}

fn on_staging_command(args: &OnStagingArgs) -> CommandChain {
    let mut c = CommandChain::new();

    c.add(Git::checkout("staging"));
    c.add(Git::pull());

    c.add(Git::merge(&args.branch));
    c.add(Git::push());
    c.add(Git::push_staging());

    c
}

#[derive(Debug)]
struct OnStagingArgs {
    branch: String,
}

impl CommandArgs for OnStagingArgs {
    fn from_args(args: &ArgMatches) -> Option<Self> {
        let branch = if let Some(branch) = args.value_of("BRANCH") {
            branch.to_string()
        } else {
            return None;
        };

        Some(OnStagingArgs { branch })
    }

    fn rerun_command(&self) -> String {
        let mut rerun_command = String::new();
        rerun_command.push_str("on-staging");
        rerun_command.push_str(&format!(" {}", self.branch));
        rerun_command
    }
}
