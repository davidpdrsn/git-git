use clap::ArgMatches;
use command_chain::*;
use commands::*;
use git::Git;

pub fn run_start(args: &ArgMatches) {
    StartArgs::parse_args_and_run_command(&args, start_command);
}

fn start_command(args: &StartArgs) -> CommandChain {
    let mut c = CommandChain::new();

    c.add(Git::checkout(&args.base));
    c.add(Git::pull());
    c.add(Git::branch(&args.branch));
    c.add(Git::checkout(&args.branch));

    if args.push {
        c.add(Git::push_and_set_upstream(&args.branch));
    }

    c
}

#[derive(Debug)]
struct StartArgs {
    branch: String,
    dry_run: bool,
    prefix: bool,
    push: bool,
    base: String,
    from_step: usize,
}

impl CommandArgs for StartArgs {
    fn from_args(args: &ArgMatches) -> Option<Self> {
        let branch = if let Some(branch) = args.value_of("BRANCH") {
            branch.into()
        } else {
            return None;
        };

        let base = if let Some(base) = args.value_of("base") {
            base.into()
        } else {
            "master".into()
        };

        let mut s = StartArgs {
            branch: branch,
            dry_run: false,
            prefix: false,
            push: false,
            base: base,
            from_step: 0,
        };

        if args.is_present("dry-run") {
            s.dry_run = true;
        }

        if args.is_present("prefix") {
            s.prefix = true;
            s.branch = format!("{}-{}", s.base, s.branch);
        }

        if args.is_present("push") {
            s.push = true;
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
        rerun_command.push_str("start");
        if self.dry_run {
            rerun_command.push_str(" --dry-run");
        }
        if self.prefix {
            rerun_command.push_str(" --prefix");
        }
        if self.push {
            rerun_command.push_str(" --push");
        }
        rerun_command.push_str(&format!(" --base {}", self.base));
        rerun_command.push_str(&format!(" {}", self.branch));
        rerun_command
    }
}
