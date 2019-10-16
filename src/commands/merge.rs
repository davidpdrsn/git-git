use crate::command_chain::*;
use crate::commands::*;
use crate::git::{branch_exists, current_branch_with_confirm, ConfirmDefault, Git};
use clap::ArgMatches;
use std::string::ToString;

pub fn run_merge(args: &ArgMatches) {
    MergeArgs::from_args(&args)
        .unwrap()
        .parse_args_and_run_command(&args, merge_command);
}

fn merge_command(args: &MergeArgs) -> CommandChain {
    let mut c = CommandChain::new();

    c.add(Git::checkout(&args.into));
    c.add(Git::pull());

    for branch in &args.branches {
        c.add(Git::checkout(&branch));
        c.add(Git::pull());

        if args.no_rebase {
            c.add(Git::checkout(&args.into));
            c.add(Git::merge(&branch));
        } else {
            c.add(Git::rebase(&args.into));
            c.add(Git::force_push());
            c.add(Git::checkout(&args.into));
            c.add(Git::fast_forward_merge(&branch));
        }
    }

    c.add(Git::checkout(&args.into));
    c.add(Git::push());

    for branch in &args.branches {
        c.add(Git::delete_branch(branch));
        c.add(Git::delete_remote_branch(branch));
        c.add(Git::prune_remote());
    }

    if args.into == "master" {
        for branch in ["staging", "develop"].iter() {
            if branch_exists(branch) {
                c.add(Git::checkout(branch));
                c.add(Git::pull());
                c.add(Git::merge(&args.into));
                c.add(Git::push());
            }
        }
    }

    c.add(Git::checkout(&args.into));

    c
}

#[derive(Debug)]
struct MergeArgs {
    no_rebase: bool,
    into: String,
    branches: Vec<String>,
}

impl MergeArgs {
    fn from_args(args: &ArgMatches) -> Option<Self> {
        let branches = if let Some(branches) = args.values_of("BRANCH") {
            branches.map(ToString::to_string).collect()
        } else {
            vec![current_branch_with_confirm(
                |current_branch| {
                    format!("Do you want to merge the current branch {}", current_branch)
                },
                ConfirmDefault::No,
            )]
        };

        let into = if let Some(into) = args.value_of("into") {
            into.into()
        } else {
            "master".into()
        };

        let mut s = MergeArgs {
            no_rebase: false,
            into,
            branches,
        };

        if args.is_present("no-rebase") {
            s.no_rebase = true;
        }

        Some(s)
    }
}

impl CommandArgs for MergeArgs {
    fn rerun_command(&self) -> String {
        let mut rerun_command = String::new();
        rerun_command.push_str("merge");
        if self.no_rebase {
            rerun_command.push_str(" --no-rebase");
        }
        rerun_command.push_str(&format!(" --into {}", self.into));
        for branch in &self.branches {
            rerun_command.push_str(&format!(" {}", branch));
        }
        rerun_command
    }
}
