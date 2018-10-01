use clap::ArgMatches;
use command_chain::*;
use commands::*;
use git::Git;

pub fn run_ship_hotfix(args: &ArgMatches) {
    ShipHotfixArgs::parse_args_and_run_command(&args, ship_hotfix_command);
}

fn ship_hotfix_command(_args: &ShipHotfixArgs) -> CommandChain {
    let mut c = CommandChain::new();

    c.add(Git::checkout("master"));
    c.add(Git::pull_rebase());
    c.add(Git::push());

    for branch in ["staging", "develop"].iter() {
        c.add(Git::checkout(branch));
        c.add(Git::pull());
        c.add(Git::merge("master"));
        c.add(Git::push());
    }

    c.add(Git::checkout("master"));

    c
}

#[derive(Debug)]
struct ShipHotfixArgs {
    dry_run: bool,
    from_step: usize,
}

impl CommandArgs for ShipHotfixArgs {
    fn from_args(args: &ArgMatches) -> Option<Self> {
        let mut s = ShipHotfixArgs {
            dry_run: false,
            from_step: 0,
        };

        if args.is_present("dry-run") {
            s.dry_run = true;
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
        rerun_command.push_str("ship-hotfix");
        if self.dry_run {
            rerun_command.push_str(" --dry-run");
        }
        rerun_command
    }
}
