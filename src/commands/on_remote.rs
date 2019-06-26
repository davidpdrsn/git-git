use crate::command_chain::*;
use crate::commands::*;
use crate::git::{current_branch_with_confirm, ConfirmDefault, Git};
use clap::ArgMatches;
use std::fmt;

pub fn run_on_remote(remote: Remote, args: &ArgMatches) {
    OnRemote::from_args(&args, remote)
        .unwrap()
        .parse_args_and_run_command(&args, on_remote_command);
}

fn on_remote_command(args: &OnRemote) -> CommandChain {
    let mut c = CommandChain::new();

    c.add(Git::push());

    c.add(Git::checkout(&args.remote.to_string()));

    c.add(Git::pull());

    c.add(Git::merge(&args.branch));
    c.add(Git::push());

    if !args.no_ship {
        match args.remote {
            Remote::Staging => c.add(Git::push_staging()),
            Remote::Develop => c.add(Git::push_develop()),
        }
    }

    c.add(Git::checkout(&args.branch));

    c
}

#[derive(Debug)]
pub enum Remote {
    Staging,
    Develop,
}

impl fmt::Display for Remote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Remote::Staging => write!(f, "staging"),
            Remote::Develop => write!(f, "develop"),
        }
    }
}

#[derive(Debug)]
struct OnRemote {
    branch: String,
    remote: Remote,
    no_ship: bool,
}

impl OnRemote {
    fn from_args(args: &ArgMatches, remote: Remote) -> Option<Self> {
        let branch = if let Some(branch) = args.value_of("BRANCH") {
            branch.to_string()
        } else {
            current_branch_with_confirm(
                |current_branch| {
                    format!(
                        "Do you want to merge {} to {} remote",
                        current_branch, remote,
                    )
                },
                ConfirmDefault::Yes,
            )
        };

        let no_ship = args.is_present("no-ship");

        Some(OnRemote {
            branch,
            remote,
            no_ship,
        })
    }
}

impl CommandArgs for OnRemote {
    fn rerun_command(&self) -> String {
        let mut rerun_command = String::new();
        rerun_command.push_str(&format!("on-{}", self.remote));
        if self.no_ship {
            rerun_command.push_str(" --no-ship");
        }
        rerun_command.push_str(&format!(" {}", self.branch));
        rerun_command
    }
}
