use clap::ArgMatches;
use command_chain::*;
use commands::*;
use git::Git;

pub fn run_on_remote(remote: Remote, args: &ArgMatches) {
    OnRemote::from_args(&args, remote)
        .unwrap()
        .parse_args_and_run_command(&args, on_remote_command);
}

fn on_remote_command(args: &OnRemote) -> CommandChain {
    let mut c = CommandChain::new();

    match args.remote {
        Remote::Staging => c.add(Git::checkout("staging")),
        Remote::Develop => c.add(Git::checkout("develop")),
    };

    c.add(Git::pull());

    c.add(Git::merge(&args.branch));
    c.add(Git::push());

    match args.remote {
        Remote::Staging => c.add(Git::push_staging()),
        Remote::Develop => c.add(Git::push_develop()),
    };

    c.add(Git::checkout(&args.branch));

    c
}

#[derive(Debug)]
pub enum Remote {
    Staging,
    Develop,
}

#[derive(Debug)]
struct OnRemote {
    branch: String,
    remote: Remote,
}

impl OnRemote {
    fn from_args(args: &ArgMatches, remote: Remote) -> Option<Self> {
        let branch = if let Some(branch) = args.value_of("BRANCH") {
            branch.to_string()
        } else {
            return None;
        };

        Some(OnRemote { branch, remote })
    }
}

impl CommandArgs for OnRemote {
    fn rerun_command(&self) -> String {
        let mut rerun_command = String::new();
        rerun_command.push_str("on-staging");
        rerun_command.push_str(&format!(" {}", self.branch));
        rerun_command
    }
}
