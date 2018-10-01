use clap::ArgMatches;
use command_chain::*;
use commands::*;
use git::Git;
use std::string::ToString;

pub fn run_merge(args: &ArgMatches) {
    MergeArgs::parse_args_and_run_command(&args, merge_command);
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
            c.add(Git::checkout(branch));
            c.add(Git::pull());
            c.add(Git::merge(&args.into));
            c.add(Git::push());
        }
    }

    c.add(Git::checkout(&args.into));

    c
}

#[derive(Debug)]
struct MergeArgs {
    dry_run: bool,
    no_rebase: bool,
    from_step: usize,
    into: String,
    branches: Vec<String>,
}

impl CommandArgs for MergeArgs {
    fn from_args(args: &ArgMatches) -> Option<Self> {
        let branches = if let Some(branches) = args.values_of("BRANCH") {
            branches.map(ToString::to_string).collect()
        } else {
            return None;
        };

        let into = if let Some(into) = args.value_of("into") {
            into.into()
        } else {
            "master".into()
        };

        let mut s = MergeArgs {
            dry_run: false,
            no_rebase: false,
            from_step: 0,
            into,
            branches,
        };

        if args.is_present("dry-run") {
            s.dry_run = true;
        }

        if args.is_present("no-rebase") {
            s.no_rebase = true;
        }

        args.value_of("from-step")
            .and_then(|step| step.parse().ok())
            .map(|step| s.from_step = step);

        Some(s)
    }

    fn dry_run(&self) -> bool {
        self.dry_run
    }

    fn from_step(&self) -> usize {
        self.from_step
    }

    fn rerun_command(&self) -> String {
        let mut rerun_command = String::new();
        rerun_command.push_str("merge");
        if self.dry_run {
            rerun_command.push_str(" --dry-run");
        }
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
